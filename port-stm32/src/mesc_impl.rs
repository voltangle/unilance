use crate::{bsp, cpu_usage};
use core::mem;
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_time::Duration;
use mesc::{MESC_motor_typedef, Motor};

// Intentionally blank, only there to satisfy the linker (MESCfoc_Init is not even called)
#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_1(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_2(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_3(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
pub extern "C" fn hw_init(motor: &mut MESC_motor_typedef) {}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn MESC_getHallState() -> i32 {
    bsp::foc::get_hall_state() as i32
}

// TODO: Make the naming convention correct

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn getRawADC() {
    bsp::foc::refresh_adc();
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn getRawADCVph() {
    bsp::foc::refresh_adc_for_vphase();
}

// Set from main
pub static HCLK_HZ: AtomicU32 = AtomicU32::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn MESChal_delayMs(ms: u32) {
    // Blocking delay implemented by Embassy's time driver.
    // This does not require async/await at the call site.
    embassy_time::block_for(Duration::from_millis(ms as u64));
}

#[unsafe(no_mangle)]
pub extern "C" fn MESChal_getTimerHz() -> u32 {
    // FIXME: Doesn't take into account the timer prescaler
    HCLK_HZ.load(Ordering::Relaxed)
}

// SAFETY: this function is retrieving the pointer to the enclosing mesc::Motor
// struct from the rs_motor pointer in MESC_motor_typedef. Pointers should be the same
// size, so transmute should be fine.
fn c_motor_as_rs(motor: &mut MESC_motor_typedef) -> &mut Motor {
    unsafe { mem::transmute(motor) }
}

// TODO: Implement all these with functions from the BSP

#[unsafe(no_mangle)]
extern "C" fn MESChal_setDeadtimeNs(motor: &mut MESC_motor_typedef, ns: u16) {
    bsp::foc::set_deadtime(c_motor_as_rs(motor), ns);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_break(motor: &mut MESC_motor_typedef) {
    bsp::foc::phase_a_break(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_break(motor: &mut MESC_motor_typedef) {
    bsp::foc::phase_b_break(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_break(motor: &mut MESC_motor_typedef) {
    bsp::foc::phase_c_break(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_enable(motor: &mut MESC_motor_typedef) {
    bsp::foc::phase_a_enable(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_enable(motor: &mut MESC_motor_typedef) {
    bsp::foc::phase_b_enable(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_enable(motor: &mut MESC_motor_typedef) {
    bsp::foc::phase_c_enable(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_enableOutput(motor: &mut MESC_motor_typedef) {
    bsp::foc::enable_output(c_motor_as_rs(motor));
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    bsp::foc::phase_a_set_duty(c_motor_as_rs(motor), duty);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    bsp::foc::phase_b_set_duty(c_motor_as_rs(motor), duty);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    bsp::foc::phase_c_set_duty(c_motor_as_rs(motor), duty);
}

// TODO: This is something that is not supposed to happen.
// This is only here so for one usecase the timer continues generating update interrupts.
// Make it work differently
#[unsafe(no_mangle)]
extern "C" fn MESChal_phD_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    bsp::foc::phase_d_set_duty(c_motor_as_rs(motor), duty);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    bsp::foc::phase_a_get_duty(c_motor_as_rs(motor))
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    bsp::foc::phase_b_get_duty(c_motor_as_rs(motor))
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    bsp::foc::phase_c_get_duty(c_motor_as_rs(motor))
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_getMaxDuty(motor: &mut MESC_motor_typedef) -> u16 {
    bsp::foc::get_max_duty(c_motor_as_rs(motor))
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_setPWMFrequency(motor: &mut MESC_motor_typedef, freq: u32) {
    bsp::foc::set_pwm_frequency(c_motor_as_rs(motor), freq);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_setIRQ(motor: &mut MESC_motor_typedef, state: bool) {
    bsp::foc::set_irq(c_motor_as_rs(motor), state);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_getCPUCycles() -> u32 {
    cpu_usage::now_cycles()
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_isTimerCountingDown(motor: &mut MESC_motor_typedef) -> bool {
    bsp::foc::is_tim_counting_down(c_motor_as_rs(motor))
}
