use embassy_stm32::timer::AdvancedInstance4Channel;
use embassy_stm32::timer::complementary_pwm::ComplementaryPwm;
use unilance_mesc::c_bind::MESC_motor_typedef;
use embassy_stm32::Peripherals;

pub unsafe fn init_1(motor: &mut MESC_motor_typedef) {}
pub unsafe fn init_2(motor: &mut MESC_motor_typedef) {}
pub unsafe fn hw_init(motor: &mut MESC_motor_typedef) {}

pub unsafe fn init_3(motor: &mut MESC_motor_typedef) {
    let mut p = Peripherals::steal();
    // let mut tim = ComplementaryPwm::new(p.TIM8);

    // TODO: Implement
}
