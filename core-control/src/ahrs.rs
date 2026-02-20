/// Spacial state of the system (absolute position, movement rates, etc etc)
///
/// Pitch and roll are absolute positions in space, *_rate ones are self-explanatory,
/// and *_accel are acceleration values in different directions. These acceleration values
/// are rotated in space to be always relative to the direction of gravity, e.g. not
/// dependent on the current orientation of the vehicle. For example, even if the vehicle
/// is tilted forward a lot, acceleration forward will always be on one axis, and not
/// split between multiple. The X axis always denotes front/back acceleration, Y axis is
/// left/right, and Z axis is up/down.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpacialState {
    /// Radians
    pub pitch: f32,
    /// Radians
    pub roll: f32,
    /// Rad/s
    pub pitch_rate: f32,
    /// Rad/s
    pub roll_rate: f32,
    /// Rad/s
    pub yaw_rate: f32,
    // m/s^2
    pub x_accel: f32,
    // m/s^2
    pub y_accel: f32,
    // m/s^2
    pub z_accel: f32,
}
