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
