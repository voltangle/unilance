#![no_std]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum CoreLinkNode {
    Control,
    Supervisor,
    BMS1,
    BMS2,
    BMS3,
    BMS4
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum CoreLinkMessageGroup {
    Notification,
    Command,
    CommandAck,
    CommandNack
}

/// Made specifically for CAN-FD
#[derive(Serialize, Deserialize, Debug)]
pub struct CoreLinkCANID {
    /// 3 bits
    message_group: CoreLinkMessageGroup,
    /// 5 bits
    source: CoreLinkNode,
    /// 5 bits
    dest: CoreLinkNode,
    /// 16 bits, not as an enum as you're supposed to set it to the value of the associated
    /// const in the message struct itself
    message_id: u16,
}

/// This enum contains all possible messages in the system.
/// The #[repr(u16)] is there to know the ID of each message no matter if the enum item
/// is unit or not. For more details, refer to https://github.com/rust-lang/rfcs/blob/master/text/2363-arbitrary-enum-discriminant.md
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone)]
#[repr(u16)]
pub enum CoreLinkMessage<'a> {
    Heartbeat,
    /// Usually sent from supervisor to control. The user wants the wheel to safely shut off
    /// (or there is some other reason in the system). Control can either respond with
    /// ShuttingDown, and then supervisor has to wait for Heartbeat messages to stop
    RequestShutdown(CoreLinkMessageShutdownReason),
    /// The first message any node in the system must send.
    ///
    /// Later, this message will be also used to sync mileage numbers between system
    /// components, together with serial numbers.
    Hello {
        /// Firmware in the form of a user-readable version string.
        firmware_version: &'a str,
        serial_number: &'a str,
        /// System time as a UNIX timestamp.
        system_time: u64,
        /// Total mileage that the sender has previously remembered.
        stored_total_mileage: f32,
    },
    /// Can be sent by any system node. Denotes a generic boot failure.
    // TODO: make it also contain the reason for failure
    BootFailure,
    /// Boot was successful (duh). Can be sent by any system node.
    BootSuccessful,
    /// Request current configuration for itself. The following response message entirely
    /// depends on who sent it.
    RequestConfig,
    /// Configuration specifically for control. Can only be sent by the supervisor.
    ConfigForControl,
    /// Telemetry from the control node, that is sent with a frequency of 100 Hz.
    ///
    /// Has all data that has to be synced between control and supervisor ASAP.
    ControlTelemetryFast,
    /// Telemetry from the control node, that is sent with a frequency of 10 Hz.
    ///
    /// Has everything else, aka all the data that doesn't need to be refreshed nearly as
    /// fast as in ControlTelemetryFast.
    ControlTelemetrySlow,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum CoreLinkMessageShutdownReason {
    UserRequest,
    IdleTimeout
}

// TODO: Test it it actually works
fn get_message_id(message: CoreLinkMessage) -> u16 {
    let ptr_to_message = (&message as *const CoreLinkMessage) as *const u16;
    unsafe {
        *ptr_to_message
    }
}

/// Link between two core modules, control and supervisor
#[allow(async_fn_in_trait)]
pub trait CoreLink {
    async fn core_recv(&mut self) -> CoreLinkMessage<'_>;
    // FIXME: No result type for return. For now it's like this because I couldn't be
    // bothered to figure it out yet, and for now it's only used for in memory channels,
    // which have only one reason to fail, so a timeout will catch something like this.
    async fn core_send(&mut self, msg: &CoreLinkMessage);
}
