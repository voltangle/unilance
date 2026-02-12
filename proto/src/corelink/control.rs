use int_enum::IntEnum;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

use crate::corelink::CANFD_DATA_LEN_MAX;

/// Notification message sent by control
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone, MaxSize)]
pub enum ControlNotification {
    /// Telemetry, sent with a frequency of 100 Hz.
    ///
    /// Has all data that has to be synced between control and supervisor.
    Telemetry {
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

/// For all type definitions and documentation, please refer to the [BalanceConfig]
/// struct (and all related) in core-control.
#[allow(non_camel_case_types)]
#[derive(
    Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone, IntEnum, MaxSize,
)]
#[repr(u32)]
pub enum ControlValueKey {
    PI2D_kP,
    PI2D_kPExpo,
    PI2D_kI,
    PI2D_kDFore,
    PI2D_kDAft,
    /// Refer to [BalanceConfig::setpoint_zero]
    SetpointZero,
    PI2DIntegralMax,
    PI2DIntegralMin,
    MaxPhaseCurrent,
    /// Referring to regenerative braking current
    MinPhaseCurrent,
    /// Refer to [RideAssistConfig::enable]
    RideAssistEnabled,
}

const _: () = {
    assert!(ControlNotification::POSTCARD_MAX_SIZE <= CANFD_DATA_LEN_MAX);
};
