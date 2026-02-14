//! This file has all data structures that represent the internal filesystem of UniLANCE,
//! specifically the controller supervisor component. Refer to docs/FILESYSTEM.md for more
//! information.

// TODO: make it not so retarded

#[derive(Clone, Copy)]
pub struct ControlConfig {
    pub kp: u16,
    pub kp_expo: f32,
    pub ki: u16,
    pub kd_fore: u16,
    pub kd_aft: u16,
    pub setpoint_zero: f32,
    pub integral_max: f32,
    pub integral_min: f32,

    pub max_phase_current: u16,
    pub min_phase_current: u16,
}
