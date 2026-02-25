#![no_std]

mod bindings;
mod types;

pub use bindings::{MESC_motor_typedef, hw_setup_s};
pub use types::*;

use crate::bindings::{
    MESC_PWM_IRQ_handler, MESCfoc_Init, MESCfoc_fastLoop, MESCfoc_slowLoop,
};

// NOTE: High size usage by MESC_motor_typedef is because of the logging struct inside it.
// Size can be varied by changing the LOGLENGTH define.

/// A safe abstraction over C bindings to MESC.
///
/// You can always use all fields inside [MESC_motor_typedef], but it's recommended you
/// use these "safe" functions for interacting with MESC insides.
pub trait MescMotorExt {
    fn init(&mut self);
    fn foc_update(&mut self);
    fn foc_aux_update(&mut self);
    fn get_state(&self) -> MotorState;
    fn set_raw_adc(&mut self, i_u: u16, i_v: u16, i_w: u16, v_bus: u16);
    fn request_q(&mut self, i_q: f32);
    fn request_d(&mut self, i_d: f32);
}

impl MescMotorExt for MESC_motor_typedef {
    fn init(&mut self) {
        unsafe {
            MESCfoc_Init(self);
        }
    }

    // TODO: do proper documentation
    /// Runs MESCfoc_fastLoop and MESC_PWM_IRQ_handler.
    fn foc_update(&mut self) {
        unsafe {
            MESCfoc_fastLoop(self);
            MESC_PWM_IRQ_handler(self);
        }
    }

    // TODO: do proper documentation
    /// Runs MESCfoc_slowLoop.
    fn foc_aux_update(&mut self) {
        unsafe {
            MESCfoc_slowLoop(self);
        }
    }

    fn get_state(&self) -> MotorState {
        MotorState::from(self.MotorState)
    }

    fn set_raw_adc(&mut self, i_u: u16, i_v: u16, i_w: u16, v_bus: u16) {
        self.Raw.Iu = i_u;
        self.Raw.Iv = i_v;
        self.Raw.Iw = i_w;
        self.Raw.Vbus = v_bus;
    }

    fn request_q(&mut self, i_q: f32) {
        self.FOC.Idq_req.q = i_q;
    }

    // NOTE: this one I will most likely be scrapping, as Id should be controlled by the FOC loop
    // itself, and not the balance loop/anything else
    fn request_d(&mut self, i_d: f32) {
        self.FOC.Idq_req.d = i_d;
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
