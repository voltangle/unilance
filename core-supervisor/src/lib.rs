#![no_std]

use embassy_futures::select::{Either, select};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Ticker, Timer};
use littlefs2::driver::Storage;
use littlefs2::fs::Filesystem;
use proto::corelink::{CoreLink, Message};

use crate::info::FW_VERSION;

mod info;
pub mod input;
mod storage;

// Random entries just so its filled with something
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub enum SystemState {
    #[default]
    Booting,
    WaitingForControl,
    Running,
    ShutdownRequested,
    FaultManage,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct State {
    state: SystemState,
    control_running: bool,
    file_transmission_next_sequence_id: u32,
}

pub async fn corelink_heartbeat<T: RawMutex>(
    _state: &'static Mutex<T, State>,
    link: &'static impl CoreLink,
) {
    let mut ticker = Ticker::every(Duration::from_hz(10));

    loop {
        link.core_send(Message::Heartbeat).await;
        ticker.next().await;
    }
}

pub async fn main_task(
    state: &'static Mutex<impl RawMutex, State>,
    _fs: &'static Mutex<impl RawMutex, Filesystem<'static, impl Storage>>,
    link: &'static impl CoreLink,
) {
    let mut ticker = Ticker::every(Duration::from_hz(100));

    loop {
        match state.lock().await.state {
            SystemState::Booting => {
                // For now, there is nothing to "boot" per se, so immediately send a hello
                // to control
                link.core_send(Message::Hello {
                    firmware_version: FW_VERSION.into(),
                    // No serial numbers (of course)
                    serial_number: "".into(),
                    // RTC not yet configured
                    system_time: 0,
                    // Mileage counting system not yet done
                    stored_total_mileage: 0.0,
                })
                .await;
                state.lock().await.state = SystemState::WaitingForControl;
            }
            SystemState::WaitingForControl => {
                match select(Timer::after_millis(50), link.core_recv()).await {
                    Either::First(_) => continue,
                    Either::Second(msg) => match msg {
                        Message::BootSuccessful => {
                            state.lock().await.control_running = true;
                        }
                        // TODO: Do a file transfer implementation here
                        _ => {}
                    },
                }
            }
            SystemState::Running => todo!(),
            SystemState::ShutdownRequested => todo!(),
            SystemState::FaultManage => todo!(),
        }
        ticker.next().await;
    }
}
