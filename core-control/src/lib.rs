#![no_std]

use defmt::info;
use embassy_time::{Duration, Ticker};
use mesc::{MESC_motor_typedef, MescMotorExt};
use proto::corelink::CoreLink;

use crate::ahrs::Ahrs;
use crate::balance::BalanceState;
use crate::corelink::handle_corelink;

pub mod ahrs;
pub mod balance;
mod corelink;
mod info;

// TODO: start filling something in here

// Random entries just so its filled with something
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum SystemState {
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

pub struct State {
    pub ahrs: Ahrs,
    pub state: SystemState,
    pub motor: MESC_motor_typedef,
    pub balance: BalanceState,
}

impl State {
    pub fn new() -> Self {
        Self {
            state: SystemState::Booting,
            ahrs: Ahrs::new(),
            balance: BalanceState::new(),
            motor: MESC_motor_typedef::default(),
        }
    }
}

// TODO: If more tasks will need state, redo it as a ref to a mutex
pub async fn main_task(state: &mut State, link: &mut impl CoreLink) {
    let mut ticker = Ticker::every(Duration::from_hz(100));

    loop {
        handle_corelink(state, link).await;
        ticker.next().await;
    }
}

pub fn pwm_isr(state: &mut State) {
    state.motor.foc_update();
}
