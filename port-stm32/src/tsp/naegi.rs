use super::PlatformConfig;
use crate::Irqs;
use crate::constants::ADC_CONV_TRIG_TIM8_TRGO;
use crate::mesc_impl::HCLK_HZ;
use crate::roles::control;
use core::mem;
use core::mem::MaybeUninit;
use core::sync::atomic::Ordering;
use cortex_m::prelude::_embedded_hal_Pwm;
use defmt::{info, trace};
use drivers::mpu6500::{AccelRange, GyroRange, MPU6500Driver, Vector3};
use embassy_executor::Spawner;
use embassy_stm32::adc::{
    Adc, AdcChannel, ConversionTrigger, Exten, RegularConversionMode, RingBufferedAdc,
    SampleTime,
};
use embassy_stm32::gpio::{Level, Output, OutputType, Speed};
use embassy_stm32::interrupt::typelevel::{self, Interrupt};
use embassy_stm32::interrupt::{InterruptExt, Priority};
use embassy_stm32::mode::Async;
use embassy_stm32::pac::timer::vals::Urs;
use embassy_stm32::pac::{DMA2, GPIOB, TIM8};
use embassy_stm32::peripherals::{ADC1, ADC2, ADC3, TIM2, TIM3, TIM8};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllSource,
    RtcClockSource, Sysclk,
};
use embassy_stm32::spi::Spi;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::Channel;
use embassy_stm32::timer::complementary_pwm::{ComplementaryPwm, ComplementaryPwmPin};
use embassy_stm32::timer::low_level::{self, CountingMode, MasterMode, RoundTo};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::usart::{self, Uart};
use embassy_stm32::{Config, Peripherals, gpio, interrupt, pac, spi};
use embassy_time::Timer;
use mesc::{Hal, MESC_motor_typedef, MescMotorExt};
use static_cell::StaticCell;

/*
 * BSP for the Begode ET Max and Panther electric unicycles.
 * Codename: naegi
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
 * - PA4: I_phaseA (inverted signal)
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

#[allow(unused)]
pub struct Bsp<'a> {
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

static mut BSP_PERIPH: MaybeUninit<Bsp<'static>> = MaybeUninit::uninit();

static DEFMT_SERIAL: StaticCell<embassy_stm32::usart::Uart<Async>> = StaticCell::new();

#[allow(static_mut_refs)]
fn get_bsp() -> &'static mut Bsp<'static> {
    unsafe { &mut (*BSP_PERIPH.as_mut_ptr()) }
}

/*
 * Platform functions
 */

/// Gather all peripherals required for opereration and initialize anything that
/// needs to be initialized at this point. This function has to be called ONCE on boot.
/// Peripherals initialized here have to be ONLY initialized. They have to be either off
/// or doing something "invisible", like DMA ADC.
#[allow(static_mut_refs)]
pub async fn init<'a>(p: Peripherals, _spawner: &Spawner) {
    let mut serial_config = usart::Config::default();
    serial_config.baudrate = 460_800;
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

    let mut imu = MPU6500Driver::new(
        Spi::new(
            p.SPI3,
            p.PB3,
            p.PB5,
            p.PB4,
            p.DMA1_CH5,
            p.DMA1_CH2,
            imu_spi_conf.clone(),
        ),
        Output::new(p.PA15, Level::High, Speed::VeryHigh),
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
    // Begode pushes 20 MHz SPI in their firmware and I'm yet to see a wheel with an IMU
    // error, so should be safe for me to do this too
    imu_spi_conf.frequency = Hertz::mhz(20);
    imu.spi.set_config(&imu_spi_conf).unwrap();
    info!("IMU whoami response @20MHz: {}", imu.whoami().unwrap());

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

    let mut adc1_rb = unsafe {
        adc1.into_ring_buffered(
            p.DMA2_CH0,
            &mut ADC1_DMA_BUF,
            [
                (i_phase_a, SampleTime::CYCLES112),
                (vrefint, SampleTime::CYCLES112),
                (core_temp, SampleTime::CYCLES112),
            ]
            .into_iter(),
            RegularConversionMode::Triggered(ConversionTrigger {
                channel: ADC_CONV_TRIG_TIM8_TRGO,
                // TODO: verify that RISING_EDGE is fine, and I don't need to switch to
                // BOTH_EDGES
                edge: Exten::RISING_EDGE,
            }),
        )
    };
    let mut adc2_rb = unsafe {
        adc2.into_ring_buffered(
            p.DMA2_CH3,
            &mut ADC2_DMA_BUF,
            [
                (i_phase_c, SampleTime::CYCLES112),
                (t_driver, SampleTime::CYCLES112),
            ]
            .into_iter(),
            RegularConversionMode::Triggered(ConversionTrigger {
                channel: ADC_CONV_TRIG_TIM8_TRGO,
                edge: Exten::RISING_EDGE,
            }),
        )
    };
    let mut adc3_rb = unsafe {
        adc3.into_ring_buffered(
            p.DMA2_CH1,
            &mut ADC3_DMA_BUF,
            [
                (i_battery, SampleTime::CYCLES112),
                (v_battery, SampleTime::CYCLES112),
            ]
            .into_iter(),
            RegularConversionMode::Triggered(ConversionTrigger {
                channel: ADC_CONV_TRIG_TIM8_TRGO,
                edge: Exten::RISING_EDGE,
            }),
        )
    };

    adc1_rb.start();
    adc2_rb.start();
    adc3_rb.start();

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
    // TODO: no idea if it actually does anything, but its used as the ADC sampling
    // trigger by MESC
    // motor_tim.enable(Channel::Ch4);

    // Enable the TIM8 update interrupt
    pac::TIM8.dier().modify(|w| w.set_uie(true));
    // Enable TIM8_TRGO updates, so the ADCs can see the updates
    pac::TIM8.cr2().modify(|w| w.set_mms(MasterMode::COMPARE_OC4));
    typelevel::TIM8_UP_TIM13::unpend();
    unsafe {
        typelevel::TIM8_UP_TIM13::enable();
    }
    trace!("Motor timer max duty: {}", motor_tim.get_max_duty());

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

    unsafe {
        BSP_PERIPH.write(Bsp {
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
    get_bsp().aux_loop_tim.start();
    // FIXME: temporary test
    get_bsp().poweron.set_high();

    info!("BSP peripherals initialized");
}

pub fn startup_successful() {
    get_bsp().poweron.set_high();
    get_bsp().motor_tim.set_master_output_enable(true);
}

/// Fetch data from the IMU. Returns an optional tuple, where first element is the accel
/// vector, and second is gyro vector.
// TODO: This guy can most likely be implemented in some better and uhhhh "cleaner" way,
// just have to think how exactly
pub fn get_imu_data() -> Option<(Vector3<f32>, Vector3<f32>)> {
    if let Some(meas) = get_bsp().imu.get_measurements().ok() {
        return Some((meas.accel, meas.gyro));
    }
    None
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
    control::aux_loop();

    rtos_trace::trace::isr_exit();
}

/*
 * BSP-specific functions
 */

fn adc_dma_ready_buf_slice(stream: usize, buf: &[u16]) -> &[u16] {
    let ndtr = DMA2.st(stream).ndtr().read().0 as usize;
    let half = buf.len() / 2;

    // if the amount of data to write is smaller than half the buffer, then DMA is
    // writing to the second half of the buffer
    if ndtr < half {
        &buf[..half]
    } else {
        &buf[half..]
    }
}

#[allow(static_mut_refs)]
fn adc_dma_read() {
    unsafe {
        let adc1_buf = adc_dma_ready_buf_slice(0, &ADC1_DMA_BUF);
        let adc2_buf = adc_dma_ready_buf_slice(3, &ADC2_DMA_BUF);
        let adc3_buf = adc_dma_ready_buf_slice(1, &ADC3_DMA_BUF);

        // Phase A current sensor is actually inverted
        control::get_state().motor.set_raw_adc(
            4095 - adc1_buf[0], // I_phaseA
            2048,        // Phase B doesn't have a sensor attached
            adc2_buf[0], // I_phaseC
            adc3_buf[1], // V_battery
        );
    }
}

/*
 * MESC hooks
 */

struct MotorHal;

impl Hal for MotorHal {
    // PB6, 7, 8
    fn get_hall_state() -> u8 {
        ((GPIOB.idr().read().0 >> 6) & 0b111) as u8
    }

    #[inline(always)]
    fn refresh_adc() {
        // Empty, as ADC read is implemented through DMA
    }

    #[inline(always)]
    fn refresh_adc_for_vphase() {
        // Empty, there are no phase voltage sensors on this board
    }

    #[inline(always)]
    fn set_irq(_motor: &mut MESC_motor_typedef, state: bool) {
        TIM8.dier().read().set_uie(state);
    }

    #[inline(always)]
    fn is_tim_counting_down(_motor: &mut MESC_motor_typedef) -> bool {
        // SAFETY: The Dir enum is always going to be just two values, 0 and 1. It is
        // guaranteed that it not cause any UB.
        unsafe { mem::transmute(TIM8.cr1().read().dir()) }
    }

    #[inline(always)]
    fn set_pwm_frequency(_motor: &mut MESC_motor_typedef, freq: u32) {
        get_bsp().motor_tim.set_frequency(Hertz::hz(freq));
    }

    #[inline(always)]
    fn get_max_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        get_bsp().motor_tim.get_max_duty() as u16
    }

    #[inline(always)]
    fn phase_a_get_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        get_bsp().motor_tim.get_duty(Channel::Ch1)
    }

    #[inline(always)]
    fn phase_b_get_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        get_bsp().motor_tim.get_duty(Channel::Ch2)
    }

    #[inline(always)]
    fn phase_c_get_duty(_motor: &mut MESC_motor_typedef) -> u16 {
        get_bsp().motor_tim.get_duty(Channel::Ch3)
    }

    /*
     * These guys have a clamp going on just so I remove the chance of it panicking
     * if input duty is outside the allowed range
     */

    #[inline(always)]
    fn phase_a_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        // if duty > 3500 {
        //     trace!("Duty for phase A: {}", duty);
        // }
        get_bsp().motor_tim.set_duty(
            Channel::Ch1,
            (duty as u32).clamp(0, get_bsp().motor_tim.get_max_duty()),
        );
    }

    #[inline(always)]
    fn phase_b_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        // if duty > 3500 {
        //     trace!("Duty for phase B: {}", duty);
        // }
        get_bsp().motor_tim.set_duty(
            Channel::Ch2,
            (duty as u32).clamp(0, get_bsp().motor_tim.get_max_duty()),
        );
    }

    #[inline(always)]
    fn phase_c_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        // if duty > 3500 {
        //     trace!("Duty for phase C: {}", duty);
        // }
        get_bsp().motor_tim.set_duty(
            Channel::Ch3,
            (duty as u32).clamp(0, get_bsp().motor_tim.get_max_duty()),
        );
    }

    #[inline(always)]
    fn phase_d_set_duty(_motor: &mut MESC_motor_typedef, duty: u16) {
        get_bsp().motor_tim.set_duty(Channel::Ch4, duty.into());
    }

    #[inline(always)]
    fn enable_output(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.set_master_output_enable(true);
    }

    fn phase_a_enable(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.enable(Channel::Ch1);
    }

    fn phase_b_enable(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.enable(Channel::Ch2);
    }

    fn phase_c_enable(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.enable(Channel::Ch3);
    }

    fn phase_a_break(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.disable(Channel::Ch1);
    }

    fn phase_b_break(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.disable(Channel::Ch2);
    }

    fn phase_c_break(_motor: &mut MESC_motor_typedef) {
        get_bsp().motor_tim.disable(Channel::Ch3);
    }

    fn set_deadtime(_motor: &mut MESC_motor_typedef, ns: u16) {
        // FIXME: doesn't take into account the timer prescaler
        let tim_clk = HCLK_HZ.load(Ordering::Relaxed) as f32;
        // how many nanoseconds there are in a second
        let ns_in_sec = 1000000000.0;
        // how many nanoseconds equal to one timer counter tick
        let tick_ns = ns_in_sec / tim_clk;

        get_bsp().motor_tim.set_dead_time(ns / tick_ns as u16);
    }
}

mesc::global_hal!(MotorHal);
