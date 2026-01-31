#![no_std]

use embassy_time::{Duration, Ticker};
use heapless::{Vec, VecView};
use proto::corelink::{
    CoreLink, FileTransmissionDeniedReason, FileTransmissionNackReason, Message,
};

use crate::info::FW_VERSION;

pub mod ahrs;
pub mod balance;
mod info;

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

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
enum FileTransferState {
    WaitingForFirstPacket,
    Requested,
    Transferring,
    Ended,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct FileTransfer {
    sequence_id: u32,
    packet_num: u32,
    state: FileTransferState,
}

// TODO: If more tasks will need state, redo it as a ref to a mutex
pub async fn main_task(state: &mut State, link: &mut impl CoreLink) {
    let mut ticker = Ticker::every(Duration::from_hz(100));

    let mut file_transfers: Vec<FileTransfer, 5> = Vec::new();

    loop {
        match link.core_recv().await {
            Message::Hello {
                firmware_version,
                serial_number,
                system_time,
                stored_total_mileage,
            } => {
                if firmware_version != info::FW_VERSION {
                    // TODO: Make it do an error response or something
                    panic!("Firmware version differ, unable to proceed");
                }

                link.core_send(Message::Hello {
                    firmware_version: FW_VERSION.into(),
                    serial_number: "".into(),
                    system_time: 0,
                    stored_total_mileage: 0.0,
                })
                .await;
                link.core_send(Message::ConfigRequest).await;
            }
            // FIXME: This file transfer implementation is retarded. Check if it even works
            // and redo it in a more reliable way
            Message::FileTransmissionRequest {
                filename,
                sequence_id,
                len,
            } => {
                for (i, transfer) in file_transfers.clone().iter().enumerate() {
                    // Check if there is already any transfer with that sequence
                    if transfer.sequence_id == sequence_id
                        && transfer.state != FileTransferState::Ended
                    {
                        link.core_send(Message::FileTransmissionRequestDenied {
                            reason: FileTransmissionDeniedReason::SequenceAlreadyExists,
                        })
                        .await;
                        continue;
                    }

                    if transfer.sequence_id == sequence_id
                        && transfer.state == FileTransferState::Ended
                    {
                        file_transfers.remove(i);
                    }
                }

                if file_transfers.is_full() {
                    link.core_send(Message::FileTransmissionRequestDenied {
                        reason: FileTransmissionDeniedReason::TooManyTransfers,
                    })
                    .await;
                    continue;
                }

                let transfer = FileTransfer {
                    sequence_id,
                    packet_num: todo!(),
                    state: FileTransferState::WaitingForFirstPacket,
                };
                file_transfers.push(transfer);
                link.core_send(Message::FileTransmissionRequestApproved { sequence_id })
                    .await;
            }
            Message::FileTransmissionPacket {
                sequence_id,
                packet_num,
                data,
            } => {
                match file_transfers
                    .iter_mut()
                    .filter(|ft| ft.sequence_id == sequence_id)
                    .next()
                {
                    Some(ft) => {
                        ft.state = FileTransferState::Transferring;
                        // if packet received has a number larger than stored at least by 2
                        if packet_num - 1 > ft.packet_num {
                            link.core_send(Message::FileTransmissionNack {
                                sequence_id,
                                packet_num: ft.packet_num + 1,
                                reason: FileTransmissionNackReason::LostPacket,
                            })
                            .await;
                            continue;
                        }
                        // TODO: handle saving packet data somewhere
                        ft.packet_num += 1;
                        link.core_send(Message::FileTransmissionAck {
                            sequence_id,
                            packet_num,
                        });
                    }
                    None => {
                        link.core_send(Message::FileTransmissionNack {
                            sequence_id,
                            packet_num,
                            reason: FileTransmissionNackReason::UnknownSequence,
                        })
                        .await;
                        continue;
                    }
                }
            }
            Message::FileTransmissionEnd {
                sequence_id,
                packet_num,
            } => {
                match file_transfers
                    .iter_mut()
                    .filter(|ft| ft.sequence_id == sequence_id)
                    .next()
                {
                    Some(ft) => {
                        if ft.packet_num != packet_num {
                            link.core_send(Message::FileTransmissionNack {
                                sequence_id,
                                packet_num,
                                reason: FileTransmissionNackReason::LostPacket,
                            })
                            .await;
                            continue;
                        }
                    }
                    None => {
                        link.core_send(Message::FileTransmissionNack {
                            sequence_id,
                            packet_num,
                            reason: FileTransmissionNackReason::UnknownSequence,
                        })
                        .await;
                        continue;
                    }
                }
                // TODO: handle saving data
                file_transfers.retain(|ft| ft.sequence_id != sequence_id);
            }
            _ => {}
        }
        ticker.next().await;
    }
}
