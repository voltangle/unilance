#![no_std]

mod bindings;
mod types;

pub use bindings::{MESC_motor_typedef, hw_setup_s};
pub use types::*;

use crate::bindings::{
    MESC_PWM_IRQ_handler, MESCfoc_fastLoop, MESCfoc_slowLoop,
    MESCfoc_Init
};
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
        let mut m = Self {
            motor: MESC_motor_typedef {
                id: 0,
                ..Default::default()
            },
        };
        m.motor.rs_motor = ptr::from_mut(&mut m) as *mut c_void;

        // TODO: make the check automatic
        // So, my idea is to have a list of parameters that have to be explicitly set,
        // and after the Supervisor finishes sending all configuration options it will
        // ask to "commit" the config, in this case verify it. If not all required options
        // were set, Control will respond with a nack and give a list of options that were
        // not set yet. If it's all good though, Control will set conf_is_valid and
        // allow the Supervisor to start sending commands, like starting balance or
        // something.
        // m.motor.conf_is_valid = false;

        unsafe {
            MESCfoc_Init(&mut m.motor);
        }

        return m;
    }

    // TODO: do proper documentation
    /// Runs MESCfoc_fastLoop and MESC_PWM_IRQ_handler.
    pub fn foc_update(&mut self) {
        unsafe {
            MESCfoc_fastLoop(&mut self.motor);
            MESC_PWM_IRQ_handler(&mut self.motor);
        }
    }

    // TODO: do proper documentation
    /// Runs MESCfoc_slowLoop.
    pub fn foc_aux_update(&mut self) {
        unsafe {
            MESCfoc_slowLoop(&mut self.motor);
        }
    }

    pub fn get_state(&self) -> MotorState {
        MotorState::from(self.motor.MotorState)
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

    // NOTE: this one I will most likely be scrapping, as Id should be controlled by the FOC loop
    // itself, and not the balance loop/anything else
    pub fn request_d(&mut self, i_d: f32) {
        self.motor.FOC.Idq_req.d = i_d;
    }
}

// pub struct Hardware {
//     /// Default ADC offset, aka what value is considered "zero". Applied to members of
//     /// `motor->offset`.
//     pub default_adc_offset: u16,
//     /// Max board current allowable
//     pub i_max: f32,
//     /// Max board voltage allowable
//     pub v_max: f32,
//     /// Min voltage at which we turn off PWM to avoid brownouts, nastiness.
//     pub v_min: f32,
//     /// Shunt resistance, ohms
//     pub r_shunt: f32,
//     /// Vbus top divider - Also for switch divider
//     pub rvbt: f32,
//     /// Vbus bottom divider - Also for switch divider
//     pub rvbb: f32,
//     /// == rvbb / (rvbb + rvbt); resistor divider network gain
//     pub vbgain: f32,
//     pub phase_current_pullup: f32,
//     pub phase_current_series_resistance: f32,
//     /// OpAmp gain, if external, or internal PGA
//     pub op_gain: f32,
//     /// e.g. Rshunt * OpGain * RIphPU / (RIphSR + RIphPU);
//     /// network gain network * opamp gain - total gain before the current hits the
//     /// ADC, might want this inverted to avoid using division?
//     pub i_gain: f32,
//     /// Current limit that will trigger a software
//     /// generated break from ADC. Actual current equal to
//     /// (RawCurrLim-IMid)*3.3/4096/Gain/Rshunt //example
//     /// (4096-2048)*3.3/(4096*16*0.001)= 103A
//     pub raw_curr_lim: f32,
//     /// Voltage limit that will trigger a software
//     /// generated break from ADC. Actual voltage equal to
//     /// RawVoltLim*3.3*Divider/4096            //
//     /// example 2303*3.3/4096*(R1k5+R47k/R1K5)=60V
//     pub raw_volt_lim: f32
// }
