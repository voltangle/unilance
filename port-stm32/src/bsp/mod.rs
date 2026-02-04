#[cfg(feature = "board_begode_etmax")]
mod begode_etmax;

#[cfg(feature = "board_begode_etmax")]
pub use begode_etmax::*;

// NOTE: No idea if it actually makes sense to do an extension trait, but shit, it looks
// nice when used
pub trait PlatformConfig {
    fn for_platform() -> Self;
}
