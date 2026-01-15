#![no_std]

pub mod c_bind;

use c_bind::MESC_motor_typedef;

pub fn mesc_start(motor: *mut MESC_motor_typedef) {
    unsafe {
        c_bind::ul_mesc_start(motor);
    }
}
