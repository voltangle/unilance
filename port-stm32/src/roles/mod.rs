#[cfg(not(any(feature = "role_control", feature = "role_supervisor")))]
compile_error!("Enable at least one role: role_control or role_supervisor (or both)!");

pub mod control;
pub mod supervisor;

// NOTE: Ideally, at least the traits should NOT be part of the port, and be split off,
// but I don't care enough to do that until a need for that will come

use crate::bsp::PlatformConfig;
use embassy_stm32::Config;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use proc_macros::for_role;
use proto::CoreLink;
use proto::CoreLinkMessage;

pub type CoreChannel = Channel<CriticalSectionRawMutex, CoreLinkMessage, 8>;

pub struct CanBusCoreLink;
pub struct MemChannelCoreLink<'a> {
    send_channel: &'a CoreChannel,
    recv_channel: &'a CoreChannel,
}

// TODO: Make CAN bus impl work

impl CanBusCoreLink {
    pub fn new() -> Self {
        CanBusCoreLink {}
    }
}

impl CoreLink for CanBusCoreLink {
    async fn core_recv(&mut self) -> CoreLinkMessage {
        unimplemented!()
    }

    async fn core_send(&mut self, _msg: &CoreLinkMessage) {
        unimplemented!()
    }
}

impl<'a> MemChannelCoreLink<'a> {
    pub fn new(send: &'a CoreChannel, recv: &'a CoreChannel) -> Self {
        MemChannelCoreLink {
            send_channel: &send,
            recv_channel: &recv,
        }
    }
}

impl CoreLink for MemChannelCoreLink<'_> {
    async fn core_send(&mut self, msg: &CoreLinkMessage) {
        self.send_channel.send(*msg).await
    }

    async fn core_recv(&mut self) -> CoreLinkMessage {
        self.recv_channel.receive().await
    }
}
