mod algo;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct BalanceState {
    pub config: BalanceConfig,
    pub run_config: BalanceRuntimeConfig,
    pub(crate) dt_sec: f32,

    pub(crate) setpoint: f32,
    pub(crate) pid_integral_accum: f32,

    pub(crate) rideassist: RideAssistState,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) struct RideAssistState {
    pub(crate) prev_state: RideAssistCoreState,
    pub(crate) state: RideAssistCoreState,
    pub(crate) accel_state: RideAssistAccelerationState,
    /// When transitioning states, this serves as the "starting point" for interpolation.
    pub(crate) state_transition_start_setpoint: f32,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) enum RideAssistCoreState {
    Acceleration,
    #[default]
    Idle,
    Braking,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub(crate) enum RideAssistAccelerationState {
    #[default]
    Slight,
    Hard,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct BalanceRuntimeConfig {
    /// What is regarded as the "zero" of a setpoint. Usually translated to the
    /// "pedal angle" setting for the user. Radians
    pub setpoint_zero: f32,
    pub rideassist: RideAssistRuntimeConfig,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct RideAssistRuntimeConfig {
    /// If disabled, Ride Assist calculations will be skipped in iterate().
    pub enable: bool,
}

// TODO: Figure out proper tuning settings
// What I had in mind is to make it work with "slider" adjustment, aka pedal hardness
// is not "hard, medium, soft", but rather a value from 0 to 100. Same for everything else
// that is applicable.

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct BalanceConfig {
    pub kp: u16,
    pub kp_expo: f32,
    pub ki: u16,
    pub kd_forward: u16,
    pub kd_backward: u16,

    /// Delta time, or how much time passes between ISR triggers. Microseconds
    pub dt: u16,

    pub rideassist: RideAssistConfig,

    pub integral_max: f32,
    pub integral_min: f32,

    // NOTE: When RFC 3550 (https://github.com/rust-lang/rust/issues/125687) comes out,
    // switch to one variable with the Range type
    /// Amperes
    pub out_min: u16,
    /// Amperes
    pub out_max: u16,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct RideAssistConfig {
    /// What's the differentiation point between "slight" and "hard" acceleration. m/s^2
    pub accel_power_threshold: f32,
    /// Threshold at which it transitions states to and from acceleration. m/s^2
    pub accel_state_threshold: f32,
    /// Threshold at which it transitions states to and from braking. m/s^2
    pub braking_state_threshold: f32,
    /// Thresholds are nice, but being right on them is not. This hysteresis applies
    /// to all state thresholds by adding and subtracting it from the threshold.
    /// For example, if hysteresis is 5 and threshold is 15, upper and lower thresholds
    /// will now be 10 and 20.
    pub state_hysteresis: u8,
}
