mod begode_etmax;

#[cfg(feature = "board_begode_etmax")]
pub use begode_etmax::*;

pub trait PlatformConfig {
    fn for_control() -> Self;
    fn for_supervisor() -> Self;
    fn for_combined() -> Self;
}
