#![no_std]

pub mod balance;
pub mod imu;

// TODO: start filling something in here

// Random entries just so its filled with something
pub enum State {
    Booting,
    Disarmed,
    Armed,
    Running,
    CriticalFault,
    ShuttingDown,
}
