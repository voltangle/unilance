#![no_std]

pub mod ahrs;
pub mod balance;

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
    FreespinOnStartup,
}
