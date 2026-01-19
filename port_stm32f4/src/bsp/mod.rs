#[cfg(feature = "board_begode_etmax")]
mod begode_etmax;

#[cfg(feature = "board_begode_etmax")]
pub use begode_etmax::*;

pub trait PlatformConfig {
    fn for_platform() -> Self;
}

