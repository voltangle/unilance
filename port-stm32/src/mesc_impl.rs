use crate::{tsp, cpu_usage};
use core::ffi::c_str::CStr;
use core::ffi::{c_char, c_double, c_uint};
use core::sync::atomic::{AtomicU32, Ordering};
use defmt::{debug, error, info, trace, warn};
use embassy_time::Duration;
use mesc::MESC_motor_typedef;

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn MESC_getHallState() -> i32 {
    tsp::foc::get_hall_state() as i32
}

// TODO: Make the naming convention correct

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn getRawADC() {
    tsp::foc::refresh_adc();
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn getRawADCVph() {
    tsp::foc::refresh_adc_for_vphase();
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

#[unsafe(no_mangle)]
extern "C" fn MESChal_setDeadtimeNs(motor: &mut MESC_motor_typedef, ns: u16) {
    tsp::foc::set_deadtime(motor, ns);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_break(motor: &mut MESC_motor_typedef) {
    tsp::foc::phase_a_break(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_break(motor: &mut MESC_motor_typedef) {
    tsp::foc::phase_b_break(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_break(motor: &mut MESC_motor_typedef) {
    tsp::foc::phase_c_break(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_enable(motor: &mut MESC_motor_typedef) {
    tsp::foc::phase_a_enable(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_enable(motor: &mut MESC_motor_typedef) {
    tsp::foc::phase_b_enable(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_enable(motor: &mut MESC_motor_typedef) {
    tsp::foc::phase_c_enable(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_enableOutput(motor: &mut MESC_motor_typedef) {
    tsp::foc::enable_output(motor);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    tsp::foc::phase_a_set_duty(motor, duty);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    tsp::foc::phase_b_set_duty(motor, duty);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    tsp::foc::phase_c_set_duty(motor, duty);
}

// TODO: This is something that is not supposed to happen.
// This is only here so for one usecase the timer continues generating update interrupts.
// Make it work differently
#[unsafe(no_mangle)]
extern "C" fn MESChal_phD_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
    tsp::foc::phase_d_set_duty(motor, duty);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    tsp::foc::phase_a_get_duty(motor)
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    tsp::foc::phase_b_get_duty(motor)
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    tsp::foc::phase_c_get_duty(motor)
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_getMaxDuty(motor: &mut MESC_motor_typedef) -> u16 {
    tsp::foc::get_max_duty(motor)
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_setPWMFrequency(motor: &mut MESC_motor_typedef, freq: u32) {
    tsp::foc::set_pwm_frequency(motor, freq);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_setIRQ(motor: &mut MESC_motor_typedef, state: bool) {
    tsp::foc::set_irq(motor, state);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_getCPUCycles() -> u32 {
    cpu_usage::now_cycles()
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_isTimerCountingDown(motor: &mut MESC_motor_typedef) -> bool {
    tsp::foc::is_tim_counting_down(motor)
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logTrace(string: *const c_char) {
    trace!("{:?}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logTraceDouble(num: c_double) {
    trace!("{:?}", num);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logTraceInt(num: c_uint) {
    trace!("{:?}", num);
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logDebug(string: *const c_char) {
    debug!("{:?}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logInfo(string: *const c_char) {
    info!("{:?}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logWarn(string: *const c_char) {
    warn!("{:?}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}

#[unsafe(no_mangle)]
extern "C" fn MESChal_logError(string: *const c_char) {
    error!("{:?}", unsafe { CStr::from_ptr(string).to_str().unwrap() });
}
