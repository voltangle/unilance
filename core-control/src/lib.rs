#![no_std]

use embassy_time::{Duration, Ticker};
use proto::corelink::CoreLink;

pub mod ahrs;
pub mod balance;

// TODO: start filling something in here

// Random entries just so its filled with something
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub enum SystemState {
    #[default]
    Booting,
    Disarmed,
    Armed,
    ShuttingDown,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum FaultType {
    OverleanPitch,
    OverleanRoll,
    FreespinOnStartup,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct State {
    state: SystemState,
}

// TODO: If more tasks will need state, redo it as a ref to a mutex
pub async fn main_task(state: &mut State, _link: &mut impl CoreLink) {
    let mut ticker = Ticker::every(Duration::from_hz(100));

    loop {
        ticker.next().await;
    }
}
