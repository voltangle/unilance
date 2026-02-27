use super::PlatformConfig;
use crate::Irqs;
use crate::constants::ADC_CONV_TRIG_TIM8_TRGO;
use crate::roles::control;
use ahrs::{Ahrs, Madgwick};
use core::mem::MaybeUninit;
use defmt::info;
use drivers::mpu6500::{AccelRange, GyroRange, MPU6500Driver, Measurements, Vector3};
use embassy_executor::Spawner;
use embassy_stm32::adc::{
    Adc, AdcChannel, ConversionTrigger, Exten, RegularConversionMode, RingBufferedAdc,
    SampleTime,
};
use embassy_stm32::gpio::OutputType;
use embassy_stm32::interrupt::typelevel::{self, Interrupt};
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_stm32::mode::Async;
use embassy_stm32::pac::timer::vals::Urs;
use embassy_stm32::pac::DMA2;
use embassy_stm32::peripherals::{ADC1, ADC2, ADC3, TIM2, TIM3, TIM8};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllSource,
    RtcClockSource, Sysclk,
};
use embassy_stm32::spi::Spi;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::low_level::{self, CountingMode, RoundTo};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::usart::{self, Uart};
use embassy_stm32::{Config, Peripherals, gpio, interrupt, pac, spi};
use embassy_time::Timer;
use mesc::MescMotorExt;
use static_cell::StaticCell;

/*
 * BSP for the Begode ET Max electric unicycle motherboard.
 *
 * MCU: STM32F405RG
 *
 * Peripherals used in this BSP:
 * - TIM8 on PC6,7,8, PA7, PB0,1: motor control
 * - TIM3 on PA6: Tail light WS281x
 * - TIM4 on PB9: Passive buzzer
 * - TIM5: Embassy time source
 * - TIM2: Auxiliary loop
 * - SPI3 on PB3,4,5 + SPI3 SS PA15: MPU6500 IMU
 * - USART1 on PA9,10: BLE module
 * - USART3 on PB10,11: BMS
 * - PB6, PB7, PB8: hall sensors
 * - PA12: park button
 * - PB15: power button
 * - PC4,5: TM1638
 * - PB12: Headlight
 * - PB14: poweron
 * - PC9: left DRL
 * - PA8: right DRL
 *
 * Clock sources:
 * - HSI: unused
 * - HSE: 8 MHz oscillator
 * - LSI: unused
 *
 * ADC pins:
 * - PC0: I_battery
 * - PC1: T_driver
 * - PA0: V_battery
 * - PA4: I_phaseA
 * - PA5: I_phaseC
 *
 * DMA setup:
 * - DMA1 Stream 2 on TIM3: WS281x
 * - DMA2 Stream 0 on ADC1: I_phaseA, Vrefint, core temp
 * - DMA2 Stream 1 on ADC3: I_battery, V_battery
 * - DMA2 Stream 3 on ADC2: I_phaceC, T_driver
 *
 * Custom IRQs:
 * - TIM2: Auxiliary loop
 * - ADC: MESC ADC handler
 * - TIM8_UP_TIM13: MESC PWM handler
 */

// TODO: Figure out how to do "input methods". Some wheels will have controls like Begode,
// where there is just a power button and a park button, some wheels will have something
// like LeaperKim, with power + headlight + OK + next, etc etc. I have to figure out
// how to make it all coexist

pub const STARTUP_DELAY_MS: u64 = 1500;

static mut ADC1_DMA_BUF: [u16; 6] = [0; 6];
static mut ADC2_DMA_BUF: [u16; 4] = [0; 4];
static mut ADC3_DMA_BUF: [u16; 4] = [0; 4];

static mut IMU_DATA: Measurements = Measurements {
    accel: Vector3::new(0.0, 0.0, 0.0),
    gyro: Vector3::new(0.0, 0.0, 0.0),
    temp: 0.0,
};
static mut AHRS: MaybeUninit<Madgwick<f32>> = MaybeUninit::uninit();
static mut AHRS_DATA: (f32, f32, f32) = (0.0, 0.0, 0.0);

#[allow(unused)]
pub struct BspPeripherals<'a> {
    poweron: gpio::Output<'a>,
    power_button: gpio::Input<'a>,
    park_button: gpio::Input<'a>,
    adc1: RingBufferedAdc<'a, ADC1>,
    adc2: RingBufferedAdc<'a, ADC2>,
    adc3: RingBufferedAdc<'a, ADC3>,
    motor_tim: ComplementaryPwm<'a, TIM8>,
    ws281x_tim: SimplePwm<'a, TIM3>,
    aux_loop_tim: low_level::Timer<'a, TIM2>,
    imu: MPU6500Driver<Spi<'a, Async, Master>, gpio::Output<'a>>,
}

static mut BSP_PERIPH: MaybeUninit<BspPeripherals<'static>> = MaybeUninit::uninit();

static DEFMT_SERIAL: StaticCell<embassy_stm32::usart::Uart<Async>> = StaticCell::new();

#[allow(static_mut_refs)]
fn bsp_periph() -> &'static mut BspPeripherals<'static> {
    unsafe { &mut (*BSP_PERIPH.as_mut_ptr()) }
}

/// Gather all peripherals required for opereration and initialize anything that
/// needs to be initialized at this point. This function has to be called ONCE on boot.
/// Peripherals initialized here have to be ONLY initialized. They have to be either off
/// or doing something "invisible", like DMA ADC.
#[allow(static_mut_refs)]
pub async fn init<'a>(p: Peripherals, spawner: &Spawner) {
    unsafe {
        AHRS.write(Madgwick::new(1.0 / 500.0, 0.1));
    }
    let mut serial_config = usart::Config::default();
    serial_config.baudrate = 115200;
    let serial = Uart::new(
        p.USART1,
        p.PA10,
        p.PA9,
        Irqs,
        p.DMA2_CH7,
        p.DMA2_CH5,
        serial_config,
    )
    .unwrap();

    defmt_serial::defmt_serial(DEFMT_SERIAL.init(serial));
    info!("defmt-serial started");

    let mut imu_spi_conf = spi::Config::default();
    imu_spi_conf.mode = spi::MODE_0;
    imu_spi_conf.frequency = Hertz::mhz(1);

    // TODO: verify that this IMU SPI conf is enough

    let mut imu = MPU6500Driver::new(
        Spi::new(
            p.SPI3,
            p.PB3,
            p.PB5,
            p.PB4,
            p.DMA1_CH5,
            p.DMA1_CH2,
            imu_spi_conf,
        ),
        gpio::Output::new(p.PA15, gpio::Level::High, gpio::Speed::VeryHigh),
    );
    // FIXME: not great to have a hardfault here. Should instead raise a global error
    // and let it run
    imu.reset().unwrap();
    Timer::after_millis(100).await;
    imu.init().unwrap();
    imu.set_sample_rate_divider(2).unwrap();
    imu.set_gyro_range(GyroRange::Dps500).unwrap();
    imu.set_accel_range(AccelRange::Range4G).unwrap();

    Timer::after_millis(150).await;

    info!("IMU whoami response: {}", imu.whoami().unwrap());

    let i_battery = p.PC0.degrade_adc();
    let t_driver = p.PC1.degrade_adc();
    let v_battery = p.PA0.degrade_adc();
    let i_phase_a = p.PA4.degrade_adc();
    let i_phase_c = p.PA5.degrade_adc();

    let adc1 = Adc::new(p.ADC1);
    let adc2 = Adc::new(p.ADC2);
    let adc3 = Adc::new(p.ADC3);

    let vrefint = adc1.enable_vrefint().degrade_adc();
    let core_temp = adc1.enable_temperature().degrade_adc();

    // NOTE: time to convert 3 channels is 17,5 microseconds, which is plenty, considering
    // that the motor timer update interrupt runs every 100 microseconds. Technically,
    // I can just get rid of the third ADC and sample input voltage/current with the
    // first two ADCs, but no harm in using all three.
    let adc1_rb = unsafe {
        adc1.into_ring_buffered(
            p.DMA2_CH0,
            &mut ADC1_DMA_BUF,
            [
                (i_phase_a, SampleTime::CYCLES480),
                (vrefint, SampleTime::CYCLES480),
                (core_temp, SampleTime::CYCLES480),
            ]
            .into_iter(),
            RegularConversionMode::Triggered(ConversionTrigger {
                channel: ADC_CONV_TRIG_TIM8_TRGO,
                edge: Exten::RISING_EDGE,
            }),
        )
    };
    let adc2_rb = unsafe {
        adc2.into_ring_buffered(
            p.DMA2_CH3,
            &mut ADC2_DMA_BUF,
            [
                (i_phase_c, SampleTime::CYCLES480),
                (t_driver, SampleTime::CYCLES480),
            ]
            .into_iter(),
            RegularConversionMode::Triggered(ConversionTrigger {
                channel: ADC_CONV_TRIG_TIM8_TRGO,
                edge: Exten::RISING_EDGE,
            }),
        )
    };
    let adc3_rb = unsafe {
        adc3.into_ring_buffered(
            p.DMA2_CH1,
            &mut ADC3_DMA_BUF,
            [
                (i_battery, SampleTime::CYCLES480),
                (v_battery, SampleTime::CYCLES480),
            ]
            .into_iter(),
            RegularConversionMode::Triggered(ConversionTrigger {
                channel: ADC_CONV_TRIG_TIM8_TRGO,
                edge: Exten::RISING_EDGE,
            }),
        )
    };

    let mut motor_tim = ComplementaryPwm::new(
        p.TIM8,
        Some(PwmPin::new(p.PC6, OutputType::PushPull)),
        Some(ComplementaryPwmPin::new(p.PA7, OutputType::PushPull)),
        Some(PwmPin::new(p.PC7, OutputType::PushPull)),
        Some(ComplementaryPwmPin::new(p.PB0, OutputType::PushPull)),
        Some(PwmPin::new(p.PC8, OutputType::PushPull)),
        Some(ComplementaryPwmPin::new(p.PB1, OutputType::PushPull)),
        None,
        None,
        Hertz::khz(10),
        CountingMode::CenterAlignedBothInterrupts,
    );
    motor_tim.set_master_output_enable(false);
    // Enable the TIM8 update interrupt
    pac::TIM8.dier().modify(|w| w.set_uie(true));
    typelevel::TIM8_UP_TIM13::unpend();
    unsafe {
        typelevel::TIM8_UP_TIM13::enable();
    }

    let ws281x_tim = SimplePwm::new(
        p.TIM3,
        Some(PwmPin::new(p.PA6, OutputType::PushPull)),
        None,
        None,
        None,
        Hertz::khz(800),
        CountingMode::EdgeAlignedUp,
    );

    // NOTE: 500 Hz was chosen because Begode did the same and it works well enough for
    // them, so idk
    // And I also save some processing power (at the time of writing, CPU usage difference
    // between 1 kHz and 500 Hz is 73% and 65% with rudimentary IMU read logic)
    let aux_loop_tim = low_level::Timer::new(p.TIM2);
    aux_loop_tim.stop(); // can never be too cautious
    aux_loop_tim.set_frequency(Hertz::hz(500), RoundTo::Slower);
    aux_loop_tim.set_counting_mode(CountingMode::EdgeAlignedUp);
    aux_loop_tim.generate_update_event();
    aux_loop_tim.clear_update_interrupt();
    aux_loop_tim.enable_update_interrupt(true);
    aux_loop_tim.regs_core().cr1().modify(|w| {
        w.set_urs(Urs::COUNTER_ONLY);
        w.set_arpe(true)
    });
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::TIM2);
    }
    pac::Interrupt::TIM2.set_priority(Priority::P2);

    aux_loop_tim.start();

    unsafe {
        BSP_PERIPH.write(BspPeripherals {
            poweron: gpio::Output::new(p.PB14, gpio::Level::Low, gpio::Speed::Medium),
            power_button: gpio::Input::new(p.PB15, gpio::Pull::Down),
            park_button: gpio::Input::new(p.PA12, gpio::Pull::Down),
            adc1: adc1_rb,
            adc2: adc2_rb,
            adc3: adc3_rb,
            motor_tim,
            ws281x_tim,
            aux_loop_tim,
            imu,
        });
    }
    spawner.spawn(ahrs_display().unwrap());
    info!("BSP peripherals initialized");
}

#[allow(static_mut_refs)]
#[embassy_executor::task]
async fn ahrs_display() {
    unsafe {
        const RAD_TO_DEG: f32 = 57.2957795129;
        loop {
            info!(
                "Roll: {}, pitch: {}, yaw: {}",
                AHRS_DATA.0 * RAD_TO_DEG,
                AHRS_DATA.1 * RAD_TO_DEG,
                AHRS_DATA.2 * RAD_TO_DEG
            );
            Timer::after_millis(100).await;
        }
    }
}

/*
 * Clock configurations
 */

impl PlatformConfig for Config {
    /// Clock configurations here result in the following frequencies:
    ///
    /// - FCLK Cortex: 168 MHz
    /// - Cortex System timer: 168 MHz
    /// - Ethernet PTP: 168 MHz
    /// - HCLK: 168 MHz
    /// - APB1 peripherals: 42 MHz
    /// - APB1 timers: 84 MHz
    /// - APB2 peripherals: 84 MHz
    /// - APB2 timers: 168 MHz
    fn for_platform() -> Self {
        let mut conf = Config::default();

        conf.rcc.hsi = false;

        conf.rcc.hse = Some(Hse {
            freq: Hertz::mhz(8),
            mode: HseMode::Oscillator,
        });

        conf.rcc.sys = Sysclk::PLL1_P;
        conf.rcc.pll_src = PllSource::HSE;
        conf.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV2),
            divq: None,
            divr: None,
        });

        conf.rcc.ahb_pre = AHBPrescaler::DIV1;
        conf.rcc.apb1_pre = APBPrescaler::DIV4;
        conf.rcc.apb2_pre = APBPrescaler::DIV2;
        conf.rcc.mux.rtcsel = RtcClockSource::HSE;

        conf
    }
}

/*
 * Interrupts
 */

#[interrupt]
fn TIM8_UP_TIM13() {
    rtos_trace::trace::isr_enter();

    adc_dma_read();
    control::motor_loop();

    // Clear update flag
    pac::TIM8.sr().modify(|w| w.set_uif(false));

    rtos_trace::trace::isr_exit();
}

/// The balance loop interrupt
#[allow(static_mut_refs)]
#[interrupt]
fn TIM2() {
    rtos_trace::trace::isr_enter();

    // Clear update flag
    pac::TIM2.sr().modify(|w| w.set_uif(false));

    unsafe {
        IMU_DATA = bsp_periph().imu.get_measurements().unwrap();
        AHRS_DATA = (*AHRS.as_mut_ptr())
            .update_imu(&IMU_DATA.gyro, &IMU_DATA.accel)
            .unwrap()
            .euler_angles();
    }
    control::aux_loop();

    rtos_trace::trace::isr_exit();
}

/*
 * Platform functions
 */

pub fn startup_successful() {
    bsp_periph().poweron.set_high();
    bsp_periph().motor_tim.set_master_output_enable(true);
}

/*
 * BSP-specific functions
 */

fn adc_dma_ready_buf_slice(stream: usize, buf: &[u16]) -> &[u16] {
    let ndtr = DMA2.st(stream).ndtr().read().0 as usize;

    // if the amount of data to write is smaller than half the buffer, then DMA is
    // writing to the second half of the buffer
    if ndtr < (buf.len() - 1) / 2 {
        &buf[..buf.len() / 2]
    } else {
        &buf[(buf.len() / 2) - 1..]
    }
}

#[allow(static_mut_refs)]
fn adc_dma_read() {
    unsafe {
        let adc1_buf = adc_dma_ready_buf_slice(0, &ADC1_DMA_BUF);
        let adc2_buf = adc_dma_ready_buf_slice(3, &ADC2_DMA_BUF);
        let adc3_buf = adc_dma_ready_buf_slice(1, &ADC3_DMA_BUF);

        control::get_state().motor.set_raw_adc(
            adc1_buf[0], // I_phaseA
            2048,        // Phase B doesn't have a sensor attached
            adc2_buf[1], // I_phaseC
            adc3_buf[0], // V_battery
        );
    }
}

/*
 * MESC hooks
 */

pub mod foc {
    use crate::bsp::begode_etmax::bsp_periph;
    use crate::mesc_impl::HCLK_HZ;
    use core::mem;
    use core::sync::atomic::Ordering;
    use cortex_m::prelude::_embedded_hal_Pwm;
    use embassy_stm32::pac::{GPIOB, TIM8};
    use embassy_stm32::time::Hertz;
    use embassy_stm32::timer::Channel;
    use mesc::MESC_motor_typedef;

    // PB6, 7, 8
    pub fn get_hall_state() -> u8 {
        ((GPIOB.idr().read().0 >> 6) & 0b111) as u8
    }

    #[inline(always)]
    pub fn refresh_adc() {
        // Empty, as ADC read is implemented through DMA
    }

    #[inline(always)]
    pub fn refresh_adc_for_vphase() {
        // Empty, there are no phase voltage sensors on this board
    }

    #[inline(always)]
    pub fn set_irq(_motor: &mut MESC_motor_typedef, state: bool) {
        TIM8.dier().read().set_uie(state);
    }

    #[inline(always)]
    pub fn is_tim_counting_down(_motor: &mut MESC_motor_typedef) -> bool {
        // The Dir enum is always going to be just two values, 0 and 1. It is guaranteed
        // that it not cause any UB.
        unsafe { mem::transmute(TIM8.cr1().read().dir()) }
    }

    #[inline(always)]
    pub fn set_pwm_frequency(_motor: &mut MESC_motor_typedef, freq: u32) {
        bsp_periph().motor_tim.set_frequency(Hertz::hz(freq));
    }

    #[inline(always)]
    pub fn get_max_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        bsp_periph().motor_tim.get_max_duty() as u16
    }

    #[inline(always)]
    pub fn phase_a_get_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        bsp_periph().motor_tim.get_duty(Channel::Ch1)
    }

    #[inline(always)]
    pub fn phase_b_get_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        bsp_periph().motor_tim.get_duty(Channel::Ch2)
    }

    #[inline(always)]
    pub fn phase_c_get_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        bsp_periph().motor_tim.get_duty(Channel::Ch3)
    }

    #[inline(always)]
    pub fn phase_a_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        bsp_periph().motor_tim.set_duty(Channel::Ch1, duty.into());
    }

    #[inline(always)]
    pub fn phase_b_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        bsp_periph().motor_tim.set_duty(Channel::Ch2, duty.into());
    }

    #[inline(always)]
    pub fn phase_c_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        bsp_periph().motor_tim.set_duty(Channel::Ch3, duty.into());
    }

    #[inline(always)]
    pub fn phase_d_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        bsp_periph().motor_tim.set_duty(Channel::Ch4, duty.into());
    }

    #[inline(always)]
    pub fn enable_output(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.set_master_output_enable(true);
    }

    pub fn phase_a_enable(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.enable(Channel::Ch1);
    }

    pub fn phase_b_enable(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.enable(Channel::Ch2);
    }

    pub fn phase_c_enable(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.enable(Channel::Ch3);
    }

    pub fn phase_a_break(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.disable(Channel::Ch1);
    }

    pub fn phase_b_break(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.disable(Channel::Ch2);
    }

    pub fn phase_c_break(_motor: &mut MESC_motor_typedef) {
        bsp_periph().motor_tim.disable(Channel::Ch3);
    }

    pub fn set_deadtime(_motor: &mut MESC_motor_typedef, ns: u16) {
        // FIXME: doesn't take into account the timer prescaler
        let tim_clk = HCLK_HZ.load(Ordering::Relaxed) as f32;
        // how many nanoseconds there are in a second
        let ns_in_sec = 1000000000.0;
        // how many nanoseconds equal to one timer counter tick
        let tick_ns = ns_in_sec / tim_clk;

        bsp_periph().motor_tim.set_dead_time(ns / tick_ns as u16);
    }
}
