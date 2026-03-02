use ahrs::{Ahrs as ExternalAhrs, Mahony};
use defmt::Format;
use nalgebra::Vector3;

pub struct Ahrs {
    // NOTE: this is done with a separate struct so I can extend the AHRS impl in any way
    // I want to
    ahrs: Mahony<f32>,
}

#[derive(Format, Debug)]
pub enum AhrsError {
    AccelerometerNormZero,
}

// TODO: add some way to pass in tuning parameters
impl Ahrs {
    pub fn new() -> Self {
        Self {
            ahrs: Mahony::new(1.0 / 500.0, 0.5, 0.0),
        }
    }

    /// Consumes raw IMU data and:
    /// - passes it through filters
    /// - runs it through an AHRS algorithm
    /// - rotates acceleration values for SpacialState _accel values.
    ///
    /// This data is to be later consumed by the balancing algorithm.
    pub fn update(
        &mut self,
        accel: &Vector3<f32>,
        gyro: &Vector3<f32>,
    ) -> Result<SpacialState, AhrsError> {
        let data = self
            .ahrs
            .update_imu(gyro, accel)
            .map_err(|_| AhrsError::AccelerometerNormZero)?;
        let euler = data.euler_angles();

        Ok(SpacialState {
            roll: euler.0,
            pitch: euler.1,
            yaw: euler.2,
            pitch_rate: gyro[0],
            roll_rate: gyro[1],
            yaw_rate: gyro[2],
            x_accel: 0.0,
            y_accel: 0.0,
            z_accel: 0.0,
        })
    }
}

/// Spacial state of the system (absolute position, movement rates, etc etc)
///
/// Pitch and roll are absolute positions in space, *_rate ones are self-explanatory,
/// and *_accel are acceleration values in different directions. These acceleration values
/// are rotated in space to be always relative to the direction of gravity, e.g. not
/// dependent on the current orientation of the vehicle. For example, even if the vehicle
/// is tilted forward a lot, acceleration forward will always be on one axis, and not
/// split between multiple. The X axis always denotes front/back acceleration, Y axis is
/// left/right, and Z axis is up/down.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Format)]
pub struct SpacialState {
    /// Radians
    pub pitch: f32,
    /// Radians
    pub roll: f32,
    /// Radians
    pub yaw: f32,
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
