#[cfg(feature = "role_control")]
pub(crate) mod real;
pub(crate) mod stub;

#[cfg(feature = "role_control")]
pub use real::*;
#[cfg(not(feature = "role_control"))]
pub use stub::*;
