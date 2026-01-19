// use proc_macros::for_role;
// https://github.com/rust-lang/rust/issues/54727

// #[for_role("supervisor")]
#[cfg(feature = "role_supervisor")]
pub(crate) mod real;
pub(crate) mod stub;

// #[for_role("supervisor")]
#[cfg(feature = "role_supervisor")]
pub use real::*;
#[cfg(not(feature = "role_supervisor"))]
pub use stub::*;
