#[cfg(feature = "target_naegi")]
mod naegi;
#[cfg(feature = "target_naegi")]
pub use naegi::*;

#[cfg(feature = "target_kirigiri")]
mod kirigiri;
#[cfg(feature = "target_kirigiri")]
pub use kirigiri::*;

// NOTE: No idea if it actually makes sense to do an extension trait, but shit, it looks
// nice when used
pub trait PlatformConfig {
    fn for_platform() -> Self;
}
