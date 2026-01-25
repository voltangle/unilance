#![no_std]

pub mod balance;
pub mod ahrs;

// TODO: start filling something in here

// Random entries just so its filled with something
pub enum State {
    Booting,
    Disarmed,
    Armed,
    ShuttingDown,
}

pub enum FaultType {
    OverleanPitch,
    OverleanRoll,
    FreespinOnStartup
}
