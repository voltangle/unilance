#![no_std]

pub mod input;

// Random entries just so its filled with something
pub enum State {
    Booting,
    WaitingForControl,
    Running,
    ShutdownRequested,
    FaultManage,
}
