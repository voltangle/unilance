use crate::bsp;
use core::{
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};
use embassy_stm32::Peripherals;
use embassy_time::Duration;
use mesc::MESC_motor_typedef;

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_1(motor: &mut MESC_motor_typedef) {
    bsp::init_1(motor);
}

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_2(motor: &mut MESC_motor_typedef) {
    bsp::init_2(motor);
}

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_3(motor: &mut MESC_motor_typedef) {
    bsp::init_3(motor);
}

#[unsafe(no_mangle)]
pub extern "C" fn hw_init(motor: &mut MESC_motor_typedef) {
    bsp::hw_init(motor);
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn MESC_getHallState() -> i32 {
    bsp::get_hall_state() as i32
}

// TODO: Make the naming convention correct

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn getRawADC() {
    bsp::refresh_adc();
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
extern "C" fn getRawADCVph() {
    bsp::refresh_adc_for_vphase();
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
    HCLK_HZ.load(Ordering::Relaxed)
}

// TODO: Implement all these with functions from the BSP

#[unsafe(no_mangle)]
extern "C" fn MESChal_setDeadtimeNs(motor: &mut MESC_motor_typedef, ns: u32) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_break(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_break(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_break(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_enable(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_enable(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_enable(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_enableOutput(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phD_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phA_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    0
}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phB_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    0
}
#[unsafe(no_mangle)]
extern "C" fn MESChal_phC_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
    0
}
#[unsafe(no_mangle)]
extern "C" fn MESChal_getMaxDuty(motor: &mut MESC_motor_typedef) -> u16 {
    0
}
#[unsafe(no_mangle)]
extern "C" fn MESChal_setMaxDuty(motor: &mut MESC_motor_typedef, duty: u16) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_disableIRQ(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_enableIRQ(motor: &mut MESC_motor_typedef) {}
#[unsafe(no_mangle)]
extern "C" fn MESChal_getTimerPrescaler(motor: &mut MESC_motor_typedef) -> u16 {
    0
}
#[unsafe(no_mangle)]
extern "C" fn MESChal_getCPUCycles() -> u32 {
    0
}
#[unsafe(no_mangle)]
extern "C" fn MESChal_isTimerCountingDown(motor: &mut MESC_motor_typedef) -> bool {
    false
}
