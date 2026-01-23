#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// TODO: Ideally, the consumer should not even have access to C bindings to MESC. Everything
// should be wrapped and abstracted away, so that unsafe (or wrong) access to MESC internals
// will be at least heavily obstructed.

// #[inline(always)]
// pub fn set_motor(motor: MESC_motor_typedef) {
//     unsafe {
//         mtr[0] = motor;
//     }
// }
//
// #[inline(always)]
// pub fn get_motor() -> &'static mut MESC_motor_typedef {
//     unsafe { &mut mtr[0] }
// }

/// A safe abstraction over C bindings to MESC.
pub struct Motor<'a> {
    /// It is only done as a mutable reference so that it can be easily constructed
    /// in MESChal functions.
    instance: &'a mut MESC_motor_typedef,
}

impl Motor<'_> {
    pub fn foc_init(&mut self) {
        unsafe { MESCfoc_Init(self.instance) };
    }
}
