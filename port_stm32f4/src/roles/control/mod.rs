// use proc_macros::for_role;
// https://github.com/rust-lang/rust/issues/54727

// #[for_role("control")]
#[cfg(feature = "role_control")]
pub(crate) mod real;
pub(crate) mod stub;

// #[for_role("control")]
#[cfg(feature = "role_control")]
pub use real::*;
#[cfg(not(feature = "role_control"))]
pub use stub::*;
