#![no_std]

mod bindings;

pub use bindings::{MESC_motor_typedef, hw_setup_s};

use crate::bindings::{MESC_PWM_IRQ_handler, MESClrobs_Init, fastLoop};
use core::ffi::c_void;
use core::ptr;

// NOTE: High size usage by MESC_motor_typedef is because of the logging struct inside it.
// Size can be varied by changing the LOGLENGTH define.

/// A safe abstraction over C bindings to MESC.
///
/// The struct doesn't derive neither Clone or Copy because the struct is not supposed
/// to be moved around in memory, because if that happens, pointers will get absolutely
/// positively fucked ([bindings::MESC_motor_typedef] has a pointer to the [Motor] struct).
pub struct Motor {
    pub motor: MESC_motor_typedef,
}

impl Motor {
    pub fn new() -> Self {
        let mut instance = Self {
            motor: MESC_motor_typedef {
                id: 0,
                ..Default::default()
            },
        };
        instance.motor.rs_motor = ptr::from_mut(&mut instance) as *mut c_void;
        instance
    }

    pub fn init(&mut self) {
        unsafe {
            MESClrobs_Init(&mut self.motor);
        }
    }

    pub fn foc_update(&mut self) {
        unsafe {
            fastLoop(&mut self.motor);
            MESC_PWM_IRQ_handler(&mut self.motor);
        }
    }

    pub fn set_raw_adc(&mut self, i_u: u16, i_v: u16, i_w: u16, v_bus: u16) {
        self.motor.Raw.Iu = i_u;
        self.motor.Raw.Iv = i_v;
        self.motor.Raw.Iw = i_w;
        self.motor.Raw.Vbus = v_bus;
    }

    pub fn request_q(&mut self, i_q: f32) {
        self.motor.FOC.Idq_req.q = i_q;
    }

    pub fn request_d(&mut self, i_d: f32) {
        self.motor.FOC.Idq_req.d = i_d;
    }
}
