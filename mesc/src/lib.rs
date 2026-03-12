#![no_std]

extern crate self as mesc;

mod bindings;
mod types;

pub use bindings::{MESC_motor_typedef, hw_setup_s};
use defmt::trace;
use micromath::F32Ext;
/// Bind a [`Hal`] implementation to MESC's exported C hooks.
///
/// ```ignore
/// #[mesc::global_hal]
/// struct MotorHal;
/// ```
pub use proc_macros::global_hal;

/// Bind a [`CoreHal`] implementation to MESC's exported C hooks.
///
/// ```ignore
/// #[mesc::global_core_hal]
/// struct MescImpl;
/// ```
pub use proc_macros::global_core_hal;
pub use types::*;

use crate::bindings::{
    MESC_PWM_IRQ_handler, MESCfoc_Init, MESCfoc_fastLoop, MESCfoc_slowLoop,
    motor_sensor_mode_e_MOTOR_SENSOR_MODE_OPENLOOP,
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
    fn timer_write(&mut self);
    fn foc_aux_update(&mut self);
    fn get_state(&self) -> MotorState;
    fn set_raw_adc(&mut self, i_u: u16, i_v: u16, i_w: u16, v_bus: u16);
    fn set_raw_adc_offsets(&mut self, i_u_offset: u16, i_v_offset: u16, i_w_offset: u16);
    fn request_q(&mut self, i_q: f32);
    fn request_d(&mut self, i_d: f32);
}

impl MescMotorExt for MESC_motor_typedef {
    fn init(&mut self) {
        trace!("Running motor init");
        // Specs for the Sherman-L motor
        self.m.Imax = 200.0;
        self.m.Pmax = 4000.0;
        self.m.IBatmax = 200.0;
        self.m.pole_pairs = 27;
        self.m.L_D = 0.00026978;
        self.m.L_Q = 0.00034389;
        self.m.L_QD = self.m.L_Q - self.m.L_D;
        self.m.R = 0.1225;
        self.m.flux_linkage = 0.0533;
        self.m.flux_linkage_min = self.m.flux_linkage * 0.7;
        self.m.flux_linkage_max = self.m.flux_linkage * 2.0;
        self.m.flux_linkage_gain = self.m.flux_linkage.sqrt() * 10.0;
        self.m.non_linear_centering_gain = 5000.0;
        self.input_vars.max_request_Idq.q = 100.0;
        self.input_vars.min_request_Idq.q = -100.0;
        self.limits.abs_max_phase_current = 200.0;
        self.limits.abs_max_bus_voltage = 170.0;
        unsafe {
            MESCfoc_Init(self);
        }
        self.MotorSensorMode = motor_sensor_mode_e_MOTOR_SENSOR_MODE_OPENLOOP;
        self.FOC.openloop_step = 7;
    }

    // TODO: do proper documentation
    /// Runs MESCfoc_fastLoop.
    fn foc_update(&mut self) {
        unsafe {
            MESCfoc_fastLoop(self);
        }
    }

    // TODO: do proper documentation
    /// Runs MESC_PWM_IRQ_handler.
    fn timer_write(&mut self) {
        unsafe {
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
        self.FOC.Idq_prereq.q = i_q;
    }

    // NOTE: this one I will most likely be scrapping, as Id should be controlled by the FOC loop
    // itself, and not the balance loop/anything else
    fn request_d(&mut self, i_d: f32) {
        self.FOC.Idq_prereq.d = i_d;
    }

    /// Configure the current sensor raw data conversion system
    fn set_raw_adc_offsets(&mut self, i_u_offset: u16, i_v_offset: u16, i_w_offset: u16) {
        self.offset.Iu = i_u_offset as f32;
        self.offset.Iv = i_v_offset as f32;
        self.offset.Iw = i_w_offset as f32;
    }
}

pub trait Hal {
    fn get_hall_state() -> u8;
    fn refresh_adc();
    fn refresh_adc_for_vphase();
    fn set_irq(motor: &mut MESC_motor_typedef, state: bool);
    fn is_tim_counting_down(motor: &mut MESC_motor_typedef) -> bool;
    fn set_pwm_frequency(motor: &mut MESC_motor_typedef, freq: u32);
    fn get_max_duty(motor: &mut MESC_motor_typedef) -> u16;
    fn phase_a_get_duty(motor: &mut MESC_motor_typedef) -> u16;
    fn phase_b_get_duty(motor: &mut MESC_motor_typedef) -> u16;
    fn phase_c_get_duty(motor: &mut MESC_motor_typedef) -> u16;
    fn phase_a_set_duty(motor: &mut MESC_motor_typedef, duty: u16);
    fn phase_b_set_duty(motor: &mut MESC_motor_typedef, duty: u16);
    fn phase_c_set_duty(motor: &mut MESC_motor_typedef, duty: u16);
    fn phase_d_set_duty(motor: &mut MESC_motor_typedef, duty: u16);
    fn enable_output(motor: &mut MESC_motor_typedef);
    fn phase_a_enable(motor: &mut MESC_motor_typedef);
    fn phase_b_enable(motor: &mut MESC_motor_typedef);
    fn phase_c_enable(motor: &mut MESC_motor_typedef);
    fn phase_a_break(motor: &mut MESC_motor_typedef);
    fn phase_b_break(motor: &mut MESC_motor_typedef);
    fn phase_c_break(motor: &mut MESC_motor_typedef);
    fn set_deadtime(motor: &mut MESC_motor_typedef, ns: u16);
}

pub trait CoreHal {
    fn delay_ms(ms: u32);
    // TODO: remove any usage of CPU cycles from MESC, use rtos_trace in consumer code
    // instead
    fn get_cpu_cycles() -> u32;
    fn log_trace(msg: &str);
    fn log_trace_int(msg: &str, num: u32);
    fn log_trace_double(msg: &str, num: f64);
    fn log_debug(msg: &str);
    fn log_info(msg: &str);
    fn log_warn(msg: &str);
    fn log_error(msg: &str);
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
