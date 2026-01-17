use embassy_stm32::Peripherals;
use embassy_stm32::pac::timer::{TimAdv, TimGp16};
use embassy_stm32::timer::complementary_pwm::ComplementaryPwm;
use mesc::MESC_motor_typedef;
use mesc::hw_setup_s;

// Peripheral table
pub const MOTOR_TIM: TimAdv = embassy_stm32::pac::TIM8;
pub const MESC_SLOW_LOOP_TIM: TimGp16 = embassy_stm32::pac::TIM3;

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

pub unsafe fn init_1(motor: &mut MESC_motor_typedef) {}

pub unsafe fn init_2(motor: &mut MESC_motor_typedef) {}

pub unsafe fn init_3(motor: &mut MESC_motor_typedef) {
    let mut p = Peripherals::steal();
    // let mut tim = ComplementaryPwm::new(p.TIM8);

    // TODO: Implement
}

pub unsafe fn hw_init(motor: &mut MESC_motor_typedef) {
    // TODO: Implement
}
