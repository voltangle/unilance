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

// TODO: add keys
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Debug, Copy, Clone, MaxSize)]
pub enum ControlValueKey {
    PI2DkP,
    PI2DkI,
    PI2DkDf,
    PI2DkDa,
}

const _: () = {
    assert!(ControlNotification::POSTCARD_MAX_SIZE <= CANFD_DATA_LEN_MAX);
};
