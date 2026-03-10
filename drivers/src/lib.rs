#![no_std]

#[cfg(feature = "bmi160")]
pub mod bmi160;
#[cfg(feature = "mpu6500")]
pub mod mpu6500;
