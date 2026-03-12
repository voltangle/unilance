use core::mem::MaybeUninit;
use core_supervisor::{ButtonRole, InputMethods, global_input};
use drivers::bmi160::Vector3;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllSource,
    RtcClockSource, Sysclk,
};
use embassy_stm32::time::Hertz;
use embassy_stm32::{Config, Peripherals};
use mesc::{Hal, MESC_motor_typedef};

// TODO: temporarily here, yet to be properly set up
use defmt_rtt as _;

use crate::tsp::PlatformConfig;

/*
 * BSP for REDSHIFT.
 * Codename: kirigiri
 *
 * MCU: STM32F405RG
 * TODO: peripherals to be filled
 */

#[allow(unused)]
pub struct Bsp<'a> {
    poweron: Output<'a>,
}

static mut BSP_PERIPH: MaybeUninit<Bsp<'static>> = MaybeUninit::uninit();

#[allow(static_mut_refs)]
fn get_periph() -> &'static mut Tsp<'static> {
    unsafe { &mut (*BSP_PERIPH.as_mut_ptr()) }
}

/*
 * Platform functions
 */

/// Gather all peripherals required for opereration and initialize anything that
/// needs to be initialized at this point. This function has to be called ONCE on boot.
/// Peripherals initialized here have to be ONLY initialized. They have to be either off
/// or doing something "invisible", like ADC.
#[allow(static_mut_refs)]
pub async fn init<'a>(p: Peripherals, _spawner: &Spawner) {
    unsafe {
        BSP_PERIPH.write(Bsp {
            poweron: Output::new(p.PC12, Level::Low, Speed::Low),
        });
    }
}

pub fn startup_successful() {
    get_periph().poweron.set_high();
}

/// Fetch data from the IMU. Returns an optional tuple, where first element is the accel
/// vector, and second is gyro vector.
// TODO: This guy can most likely be implemented in some better and uhhhh "cleaner" way,
// just have to think how exactly
pub fn get_imu_data() -> Option<(Vector3<f32>, Vector3<f32>)> {
    // NOTE: No IMU implementation yet
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
 * MESC hooks
 */

#[mesc::global_hal]
struct MotorHal;

impl Hal for MotorHal {
    fn get_hall_state() -> u8 {
        todo!()
    }

    fn refresh_adc() {
        todo!()
    }

    fn refresh_adc_for_vphase() {
        todo!()
    }

    fn set_irq(motor: &mut MESC_motor_typedef, state: bool) {
        todo!()
    }

    fn is_tim_counting_down(motor: &mut MESC_motor_typedef) -> bool {
        todo!()
    }

    fn set_pwm_frequency(motor: &mut MESC_motor_typedef, freq: u32) {
        todo!()
    }

    fn get_max_duty(motor: &mut MESC_motor_typedef) -> u16 {
        todo!()
    }

    fn phase_a_get_duty(motor: &mut MESC_motor_typedef) -> u16 {
        todo!()
    }

    fn phase_b_get_duty(motor: &mut MESC_motor_typedef) -> u16 {
        todo!()
    }

    fn phase_c_get_duty(motor: &mut MESC_motor_typedef) -> u16 {
        todo!()
    }

    fn phase_a_set_duty(motor: &mut MESC_motor_typedef, duty: u16) {
        todo!()
    }

    fn phase_b_set_duty(motor: &mut MESC_motor_typedef, duty: u16) {
        todo!()
    }

    fn phase_c_set_duty(motor: &mut MESC_motor_typedef, duty: u16) {
        todo!()
    }

    fn phase_d_set_duty(motor: &mut MESC_motor_typedef, duty: u16) {
        todo!()
    }

    fn enable_output(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn phase_a_enable(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn phase_b_enable(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn phase_c_enable(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn phase_a_break(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn phase_b_break(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn phase_c_break(motor: &mut MESC_motor_typedef) {
        todo!()
    }

    fn set_deadtime(motor: &mut MESC_motor_typedef, ns: u16) {
        todo!()
    }
}

/*
 * Input methods
 */

#[global_input]
struct Input;

impl InputMethods for Input {
    fn is_pressed(role: ButtonRole) -> bool {
        todo!()
    }

    fn dial_relative_distance() -> i16 {
        0 // no dial
    }

    fn dial_absolute_position() -> i32 {
        0 // no dial
    }
}
