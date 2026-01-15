#![no_std]

use crate::bindings::{MESC_motor_typedef, MESCfoc_Init};

mod bindings;

pub fn mesc_init() {
    // TODO: make it run a C function that initializes everything that is needed
}
