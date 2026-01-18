pub struct IMUData {
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
