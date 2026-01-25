use super::PlatformConfig;
use crate::roles;
use core::mem::MaybeUninit;
use core_control::balance::BalanceConfig;
use core_control::balance::RideAssistConfig;
use embassy_executor::Metadata;
use embassy_executor::Spawner;
use embassy_stm32::adc;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::AdcChannel;
use embassy_stm32::adc::AnyAdcChannel;
use embassy_stm32::adc::ConversionTrigger;
use embassy_stm32::adc::Exten;
use embassy_stm32::adc::RegularConversionMode;
use embassy_stm32::adc::RingBufferedAdc;
use embassy_stm32::adc::SampleTime;
use embassy_stm32::gpio;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::interrupt;
use embassy_stm32::pac;
use embassy_stm32::pac::timer::regs::SrAdv;
use embassy_stm32::pac::DMA2;
use embassy_stm32::pac::timer::{TimAdv, TimGp16};
use embassy_stm32::pac::{ADC1, ADC2, ADC3, GPIOB};
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::peripherals::ADC2;
use embassy_stm32::peripherals::ADC3;
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::peripherals::TIM8;
use embassy_stm32::rcc::{Hse, HseMode};
use embassy_stm32::spi;
use embassy_stm32::spi::Spi;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer;
use embassy_stm32::timer::complementary_pwm::ComplementaryPwm;
use embassy_stm32::timer::complementary_pwm::ComplementaryPwmPin;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::low_level::RoundTo;
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::timer::simple_pwm::SimplePwm;
use embassy_stm32::{Config, Peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use mesc::MESC_ADC_IRQ_handler;
use mesc::MESC_PWM_IRQ_handler;
use mesc::MESC_motor_typedef;
use mesc::hw_setup_s;
use proc_macros::for_role;
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
 * - SPI1 on PB3,4,5 + SPI1 SS PA15: MPU6500 IMU
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
 * - PH0,1: 8 MHz oscillator
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
 * - TIM2: balance loop
 * - ADC: MESC ADC handler
 * - TIM8_UP_TIM13: MESC PWM handler
 */

// TODO: Figure out how to do "input methods". Some wheels will have controls like Begode,
// where there is just a power button and a park button, some wheels will have something
// like LeaperKim, with power + headlight + OK + next, etc etc. I have to figure out
// how to make it all coexist

pub const STARTUP_DELAY_MS: u64 = 1500;
// FIXME: NO BALANCE CONFIG, do at least a basic tune
pub const BALANCE_CONF: BalanceConfig = BalanceConfig {
    kp: 0,
    kp_expo: 0.0,
    ki: 0,
    kd_forward: 0,
    kd_backward: 0,
    dt: 0,
    rideassist: RideAssistConfig {
        accel_power_threshold: 0.0,
        accel_state_threshold: 0.0,
        braking_state_threshold: 0.0,
        state_hysteresis: 0,
    },
    integral_min: 0.0,
    integral_max: 0.0,
    out_min: 0,
    out_max: 0,
};

static mut ADC1_DMA_BUF: [u16; 6] = [0; 6];
static mut ADC2_DMA_BUF: [u16; 4] = [0; 4];
static mut ADC3_DMA_BUF: [u16; 4] = [0; 4];
// just enough space for smooth operation
static mut WS281X_BUF: [u16; 500] = [0; 500];

pub struct BspPeripherals<'a> {
    poweron: gpio::Output<'a>,
    power_button: gpio::Input<'a>,
    park_button: gpio::Input<'a>,
    adc1: RingBufferedAdc<'a, ADC1>,
    adc2: RingBufferedAdc<'a, ADC2>,
    adc3: RingBufferedAdc<'a, ADC3>,
    balance_loop_tim: timer::low_level::Timer<'a, TIM2>,
    motor_tim: ComplementaryPwm<'a, TIM8>,
    ws281x_tim: SimplePwm<'a, TIM3>,
}

static mut BSP_PERIPH: MaybeUninit<BspPeripherals<'static>> = MaybeUninit::uninit();

#[allow(static_mut_refs)]
fn bsp_periph() -> &'static mut BspPeripherals<'static> {
    unsafe { &mut (*BSP_PERIPH.as_mut_ptr()) }
}

// Gather all peripherals required for opereration and initialize anything that
// needs to be initialized at this point. This function has to be called ONCE on boot.
// Peripherals initialized here have to be ONLY initialized. They have to be either off
// or doing something "invisible", like DMA ADC.
#[allow(static_mut_refs)]
pub fn init<'a>(p: Peripherals, _spawner: &Spawner) {
    let i_battery = p.PC0.degrade_adc();
    let t_driver = p.PC1.degrade_adc();
    let v_battery = p.PA0.degrade_adc();
    let i_phase_a = p.PA4.degrade_adc();
    let i_phase_c = p.PA5.degrade_adc();

    let mut adc1 = Adc::new(p.ADC1);
    let mut adc2 = Adc::new(p.ADC2);
    let mut adc3 = Adc::new(p.ADC3);

    let vrefint = adc1.enable_vrefint().degrade_adc();
    let core_temp = adc1.enable_temperature().degrade_adc();

    // TODO: Revisit the cycles part, maybe make it work better
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
                // TIM8_TRGO, aka ADC conversions are driven by the motor control timer.
                // Refer to RM0090 Rev 21 (F405 reference manual) table 69 (nice) in
                // section 13.6 for details.
                channel: 0b1110,
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
                channel: 0b1110,
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
                channel: 0b1110,
                edge: Exten::RISING_EDGE,
            }),
        )
    };

    let mut balance_loop_tim = timer::low_level::Timer::new(p.TIM2);
    balance_loop_tim.set_frequency(Hertz::khz(1), RoundTo::Slower);
    balance_loop_tim.enable_update_interrupt(true);

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
        CountingMode::EdgeAlignedDown, // TODO: I will need to check what's up with this
    );
    motor_tim.set_master_output_enable(false);

    let mut ws281x_tim = SimplePwm::new(
        p.TIM3,
        Some(PwmPin::new(p.PA6, OutputType::PushPull)),
        None,
        None,
        None,
        Hertz::khz(800),
        CountingMode::EdgeAlignedUp,
    );

    let mut imu_spi_conf = spi::Config::default();
    imu_spi_conf.mode = spi::MODE_0;
    imu_spi_conf.frequency = Hertz::mhz(1);

    let mut imu_spi = Spi::new(
        p.SPI1,
        p.PB3,
        p.PB5,
        p.PB4,
        p.DMA2_CH5,
        p.DMA2_CH2,
        imu_spi_conf,
    );

    unsafe {
        BSP_PERIPH.write(BspPeripherals {
            poweron: gpio::Output::new(p.PB14, gpio::Level::Low, gpio::Speed::Medium),
            power_button: gpio::Input::new(p.PB15, gpio::Pull::Down),
            park_button: gpio::Input::new(p.PA12, gpio::Pull::Down),
            adc1: adc1_rb,
            adc2: adc2_rb,
            adc3: adc3_rb,
            balance_loop_tim,
            motor_tim,
            ws281x_tim,
        });
    }
}

/*
 * MESC configuration
 */

// NOTE: ideally this default init should be in the mesc crate
#[unsafe(export_name = "g_hw_setup")]
pub static mut HW_SETUP: hw_setup_s = hw_setup_s {
    Imax: 0.0,
    Vmax: 0.0,
    Vmin: 0.0,
    Rshunt: 0.0,
    RVBT: 0.0,
    RVBB: 0.0,
    VBGain: 0.0,
    RIphPU: 0.0,
    RIphSR: 0.0,
    OpGain: 0.0,
    Igain: 0.0,
    RawCurrLim: 0,
    RawVoltLim: 0,
};

pub fn init_1(_motor: &mut MESC_motor_typedef) {}

pub fn init_2(_motor: &mut MESC_motor_typedef) {}

pub fn init_3(_motor: &mut MESC_motor_typedef) {
    unimplemented!()
}

pub fn hw_init(_motor: &mut MESC_motor_typedef) {
    unimplemented!()
}

// PB6, 7, 8
pub fn get_hall_state() -> u8 {
    ((GPIOB.idr().read().0 >> 6) & 0b111) as u8
}

/*
 * Clock configurations
 */

// NOTE: No idea if it actually makes sense to do an extension trait, but shit, it looks
// nice when used

impl PlatformConfig for Config {
    fn for_platform() -> Self {
        let mut config = Config::default();

        config.rcc.hsi = false;
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(8),
            mode: HseMode::Bypass,
        });
        config
    }
}

/*
 * Interrupts
 */

#[interrupt]
fn TIM8_UP_TIM13() {
    rtos_trace::trace::isr_enter();

    let motor = crate::get_motor();
    adc_dma_read();
    unsafe {
        mesc::fastLoop(motor);
        MESC_PWM_IRQ_handler(motor);
    }

    // Clear update flag
    pac::TIM8.sr().modify(|w| w.set_uif(false));

    rtos_trace::trace::isr_exit();
}

/// The balance loop interrupt
#[interrupt]
fn TIM2() {
    rtos_trace::trace::isr_enter();
    roles::control::balance_loop();
    rtos_trace::trace::isr_exit();
}

/*
 * Platform functions
 */

pub fn startup_successful() {
    bsp_periph().poweron.set_high();
    bsp_periph().motor_tim.set_master_output_enable(true);
}

pub fn refresh_adc() {
    // Empty, as ADC read is implemented through DMA
}

pub fn refresh_adc_for_vphase() {
    // Empty, there are no phase voltage sensors on this board
}

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
        let mut motor = crate::get_motor();
        motor.Raw.Iv = 2048;

        let adc1_buf = adc_dma_ready_buf_slice(0, &ADC1_DMA_BUF);
        motor.Raw.Iu = adc1_buf[0]; // I_phaseA

        let adc2_buf = adc_dma_ready_buf_slice(3, &ADC2_DMA_BUF);
        motor.Raw.Vbus = adc2_buf[1]; // V_battery

        let adc3_buf = adc_dma_ready_buf_slice(1, &ADC3_DMA_BUF);
        motor.Raw.Iw = adc3_buf[0];
    }
}
