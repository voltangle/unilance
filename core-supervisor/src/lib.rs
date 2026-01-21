#![no_std]

// TODO: start filling something in here

// Random entries just so its filled with something
pub enum State {
    Booting,
    WaitingForControl,
    Running,
    ShutdownRequested,
    FaultManage,
}
