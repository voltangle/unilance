mod algo;

#[derive(Default, Debug, Clone, Copy)]
pub struct BalanceState {
    pub config: BalanceConfig,
    dt_secs: f32,

    /*
     * PID stuff
     */
    setpoint: f32,
    integral_accum: f32,

    /*
     * Ride Assist
     */
    pub(crate) rideassist_prev_state: RideAssistState,
    pub(crate) rideassist_state: RideAssistState,
    pub(crate) rideassist_accel_hysteresis_state: HysteresisState,
}

/// Which hysteresis threshold is considered active
#[derive(Default, Debug, Clone, Copy)]
enum HysteresisState {
    #[default]
    Upper,
    Lower,
}

#[derive(Default, Debug, Clone, Copy)]
enum RideAssistState {
    Acceleration,
    #[default]
    Idle,
    Braking,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct BalanceConfig {
    pub kp: u16,
    pub kp_expo: f32,
    pub ki: u16,
    pub kd_forward: u16,
    pub kd_backward: u16,

    /// What is regarded as the "zero" of a setpoint. Usually translated to the
    /// "pedal angle" setting for the user. Radians
    pub setpoint_zero: f32,
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

#[derive(Default, Debug, Clone, Copy)]
pub struct RideAssistConfig {
    /// If disabled, Ride Assist calculations will be skipped in iterate().
    pub enable: bool,
    /// What's the differentiation point between "slight" and "hard" acceleration.
    pub(crate) acceleration_power_threshold: f32,
    /// Threshold at which it transitions states to and from acceleration.
    pub(crate) acceleration_state_threshold: f32,
    /// Threshold at which it transitions states to and from braking.
    pub(crate) braking_state_threshold: f32,
    /// Thresholds are nice, but being right on them is not. This hysteresis applies
    /// to both state thresholds by adding and subtracting it from the threshold.
    /// For example if hysteresis is 5 and threshold is 15, upper and lower thresholds
    /// will now be 10 and 20.
    pub(crate) state_hysteresis: u8,
}
