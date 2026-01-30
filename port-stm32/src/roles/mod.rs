#[cfg(not(any(feature = "role_control", feature = "role_supervisor")))]
compile_error!("Enable at least one role: role_control or role_supervisor (or both)!");

#[cfg(feature = "role_control")]
pub mod control;
#[cfg(feature = "role_supervisor")]
pub mod supervisor;

// NOTE: Ideally, at least the traits should NOT be part of the port, and be split off,
// but I don't care enough to do that until a need for that will come

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use proto::corelink::{CoreLink, Message};

pub type CoreChannel = Channel<CriticalSectionRawMutex, Message, 8>;

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
    async fn core_recv(&self) -> Message {
        unimplemented!()
    }

    async fn core_send(&self, _msg: Message) {
        unimplemented!()
    }
}

impl<'a> MemChannelCoreLink<'a> {
    pub fn new(send: &'a CoreChannel, recv: &'a CoreChannel) -> Self {
        MemChannelCoreLink {
            send_channel: send,
            recv_channel: recv,
        }
    }
}

impl CoreLink for MemChannelCoreLink<'_> {
    async fn core_send(&self, msg: Message) {
        self.send_channel.send(msg).await
    }

    async fn core_recv(&self) -> Message {
        self.recv_channel.receive().await
    }
}
