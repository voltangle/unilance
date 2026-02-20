use proto::corelink::control::ControlValueKey;
use proto::corelink::{CoreLink, Message, ValueNackReason};

use crate::{State, info};

pub(crate) async fn handle_corelink(state: &mut State, link: &mut impl CoreLink) {
    match link.core_recv().await {
        Message::Hello {
            firmware_version,
            serial_number: _,
            system_time: _,
            stored_total_mileage: _,
        } => {
            if firmware_version != info::FW_VERSION {
                // TODO: Make it do an error response or something
                panic!("Firmware version differ, unable to proceed");
            }
        }
        Message::IntroduceYourselves => {
            link.core_send(Message::Hello {
                firmware_version: info::FW_VERSION.into(),
                // TODO: temp variables here
                serial_number: "".into(),
                system_time: 0,
                stored_total_mileage: 0.0,
            })
            .await;
        }
        Message::WriteValue { key, value } => match ControlValueKey::try_from(key) {
            Ok(key) => match handle_write(state, key, &value) {
                Ok(_) => {
                    link.core_send(Message::WriteValueAck { key: key.into() })
                        .await
                }
                Err(_) => {
                    link.core_send(Message::WriteValueNack {
                        key: key.into(),
                        reason: ValueNackReason::BadPayload,
                    })
                    .await
                }
            },
            Err(_) => {
                link.core_send(Message::WriteValueNack {
                    key: key,
                    reason: ValueNackReason::NoSuchKey,
                })
                .await;
            }
        },
        Message::ReadValue { key } => match ControlValueKey::try_from(key) {
            Ok(key) => {
                match get_value_by_key(state, key) {
                    Ok(value) => {
                        link.core_send(Message::ReadValueAck {
                            key: key.into(),
                            value: value,
                        })
                        .await;
                    }
                    // if we get to this error branch, then something has gone terribly wrong
                    Err(_) => {
                        link.core_send(Message::ReadValueNack {
                            key: key.into(),
                            reason: ValueNackReason::Unspecified,
                        })
                        .await
                    }
                }
            }
            Err(_) => {
                link.core_send(Message::ReadValueNack {
                    key: key.into(),
                    reason: ValueNackReason::NoSuchKey,
                })
                .await
            }
        },

        _ => {}
    }
}

fn handle_write(
    state: &mut State,
    key: ControlValueKey,
    value: &[u8; 58],
) -> Result<(), postcard::Error> {
    match key {
        ControlValueKey::PI2D_kP => {
            state.balance.config.kp = postcard::from_bytes(value)?;
        }
        ControlValueKey::PI2D_kPExpo => {
            state.balance.config.kp_expo = postcard::from_bytes(value)?;
        }
        ControlValueKey::PI2D_kI => {
            state.balance.config.ki = postcard::from_bytes(value)?;
        }
        ControlValueKey::PI2D_kDFore => {
            state.balance.config.kd_fore = postcard::from_bytes(value)?;
        }
        ControlValueKey::PI2D_kDAft => {
            state.balance.config.kd_aft = postcard::from_bytes(value)?;
        }
        ControlValueKey::SetpointZero => {
            state.balance.config.setpoint_zero = postcard::from_bytes(value)?;
        }
        ControlValueKey::PI2DIntegralMax => {
            state.balance.config.integral_max = postcard::from_bytes(value)?;
        }
        ControlValueKey::PI2DIntegralMin => {
            state.balance.config.integral_min = postcard::from_bytes(value)?;
        }
        ControlValueKey::MaxPhaseCurrent => {
            // FIXME: doesn't change any settings in MESC, should do that
            state.balance.config.out_max = postcard::from_bytes(value)?;
        }
        ControlValueKey::MinPhaseCurrent => {
            // FIXME: doesn't change any settings in MESC, should do that
            state.balance.config.out_min = postcard::from_bytes(value)?;
        }
        ControlValueKey::RideAssistEnabled => {
            state.balance.config.rideassist.enable = postcard::from_bytes(value)?;
        }
    }
    Ok(())
}

fn get_value_by_key(
    state: &mut State,
    key: ControlValueKey,
) -> Result<[u8; 58], postcard::Error> {
    let mut buf: [u8; 58] = [0; 58];
    match key {
        ControlValueKey::PI2D_kP => {
            postcard::to_slice(&state.balance.config.kp, &mut buf)?;
        }
        ControlValueKey::PI2D_kPExpo => {
            postcard::to_slice(&state.balance.config.kp_expo, &mut buf)?;
        }
        ControlValueKey::PI2D_kI => {
            postcard::to_slice(&state.balance.config.ki, &mut buf)?;
        }
        ControlValueKey::PI2D_kDFore => {
            postcard::to_slice(&state.balance.config.kd_fore, &mut buf)?;
        }
        ControlValueKey::PI2D_kDAft => {
            postcard::to_slice(&state.balance.config.kd_aft, &mut buf)?;
        }
        ControlValueKey::SetpointZero => {
            postcard::to_slice(&state.balance.config.setpoint_zero, &mut buf)?;
        }
        ControlValueKey::PI2DIntegralMax => {
            postcard::to_slice(&state.balance.config.integral_max, &mut buf)?;
        }
        ControlValueKey::PI2DIntegralMin => {
            postcard::to_slice(&state.balance.config.integral_min, &mut buf)?;
        }
        ControlValueKey::MaxPhaseCurrent => {
            postcard::to_slice(&state.balance.config.out_max, &mut buf)?;
        }
        ControlValueKey::MinPhaseCurrent => {
            postcard::to_slice(&state.balance.config.out_min, &mut buf)?;
        }
        ControlValueKey::RideAssistEnabled => {
            postcard::to_slice(&state.balance.config.rideassist.enable, &mut buf)?;
        }
    }
    Ok(buf)
}
