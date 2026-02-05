//! This file has the entirety of the CORElink protocol used in UniLANCE for communication
//! between different nodes in the system.

use heapless::String;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

const CANFD_DATA_LEN_MAX: usize = 64;

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum Node {
    Control,
    Supervisor,
    BMS1,
    BMS2,
    BMS3,
    BMS4,
}

/// This enum is also used in the CAN-FD protocol as a "priority" field. If something has
/// to have high priority on the CAN bus, it should be put higher in the enum variant
/// list, so it has a smaller value, and in turn higher priority on the CAN-FD bus.
// TODO: Review this
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone)]
pub enum MessageGroup {
    Notification,
    Command,
    CommandAck,
    CommandNack,
}

/// Made specifically for CAN-FD
#[derive(Serialize, Deserialize, Debug)]
pub struct CanID {
    /// 3 bits
    message_group: MessageGroup,
    /// 5 bits
    source: Node,
    /// 5 bits
    dest: Node,
    /// 16 bits, not as an enum as you're supposed to set it to the value of the message
    /// itself, as its set as #[repr(u16)]
    message_id: u16,
}

/// This enum contains all possible messages in the system.
/// The #[repr(u16)] is there to know the ID of each message no matter if the enum item
/// is unit or not. For more details, refer to https://github.com/rust-lang/rfcs/blob/master/text/2363-arbitrary-enum-discriminant.md
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Clone, MaxSize)]
#[repr(u16)]
pub enum Message {
    Heartbeat,
    /// Usually sent from supervisor to control. The user wants the wheel to safely shut off
    /// (or there is some other reason in the system). Control can either respond with
    /// ShuttingDown, and then supervisor has to wait for Heartbeat messages to stop
    RequestShutdown(ShutdownReason),
    /// The first message any node in the system must send.
    ///
    /// Later, this message will be also used to sync mileage numbers between system
    /// components, together with serial numbers.
    Hello {
        /// Firmware in the form of a user-readable version string.
        firmware_version: String<16>,
        serial_number: String<28>,
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
    WriteValue {
        key: String<20>,
        #[serde(with = "serde_arrays")]
        value: [u8; 42],
    },
    /// Request to transmit a file from one node to another. The other node has to respond
    /// with either FileTransmissionStartApproved or FileTransmissionStartDenied.
    FileTransmissionRequest {
        /// This funny ass length was specifically chosen so it fits exactly into the 64
        /// byte limit of CAN-FD.
        filename: String<47>,
        sequence_id: u32,
        len: u64,
    },
    FileTransmissionRequestApproved {
        /// The sequence ID this transfer will use.
        sequence_id: u32,
    },
    FileTransmissionRequestDenied {
        reason: FileTransmissionDeniedReason,
    },
    FileTransmissionPacket {
        /// The ID of the entire sequence. Used for separating different file transmissions
        /// from each other.
        sequence_id: u32,
        /// The packet number. If it's the first packet sent, it would have a value of 0,
        /// if it's the second, then 1, etc etc. The reason for its existence is so
        /// if the receiving party responds with a nack, then the packet is sent again by
        /// the transmitting party.
        packet_num: u32,
        /// Sized right up the CAN-FD limit (64 bytes).
        #[serde(with = "serde_arrays")]
        data: [u8; 53],
    },
    /// The file packet was received successfully, continue on with the transmission
    FileTransmissionAck {
        sequence_id: u32,
        packet_num: u32,
    },
    /// The file packet was NOT received successfully. The sender has to send that packet
    /// again.
    FileTransmissionNack {
        sequence_id: u32,
        packet_num: u32,
        reason: FileTransmissionNackReason,
    },
    /// Marks the file transmission as "finished" at packet_num. This message should be
    /// sent by the transmitting party, which should be then followed by FileTransmissionAck
    /// from the receiving party with packet_num set to packet_num in this message.
    FileTransmissionEnd {
        sequence_id: u32,
        packet_num: u32,
    },
    /// Acknowledges that the file transmission has ended.
    FileTransmissionEndAck {
        sequence_id: u32,
    },
    /// Telemetry from the control node, that is sent with a frequency of 100 Hz.
    ///
    /// Has all data that has to be synced between control and supervisor.
    ControlTelemetry {
        /// Current speed in cm/s.
        ///
        /// The reason it's using this UoM is because I want to use SI units as much as
        /// possible, and then convert them to whatever they have to be for the end usecase. In
        /// this case, it was also supposed to be m/s, but instead of using a float to represent
        /// decimals I would just put cm/s and then divide by 100 to get m/s.
        speed_cmps: u16,
        /// Input current in deciamps.
        i_input_da: i16,
    },
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone, MaxSize)]
pub enum FileTransmissionDeniedReason {
    UnknownFile,
    NotEnoughSpace,
    SequenceAlreadyExists,
    TooManyTransfers,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone, MaxSize)]
pub enum ShutdownReason {
    UserRequest,
    IdleTimeout,
}

#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone, MaxSize)]
pub enum FileTransmissionNackReason {
    UnknownSequence,
    LostPacket,
}

impl Message {
    // TODO: Test it it actually works
    pub fn message_id(&self) -> u16 {
        let ptr_to_message = (self as *const Self) as *const u16;
        unsafe { *ptr_to_message }
    }
}

/// Link between two core modules, control and supervisor
#[allow(async_fn_in_trait)]
pub trait CoreLink {
    async fn core_recv(&self) -> Message;
    // FIXME: No result type for return. For now it's like this because I couldn't be
    // bothered to figure it out yet, and for now it's only used for in memory channels,
    // which have only one reason to fail, so a timeout will catch something like this.
    async fn core_send(&self, msg: Message);
}

/// Makes sure that the message can and will fit inside a CAN-FD packet.
const _: () = {
    assert!(Message::POSTCARD_MAX_SIZE <= CANFD_DATA_LEN_MAX);
};
