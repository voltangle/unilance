#[cfg(feature = "role_supervisor")]
pub(crate) mod real;
pub(crate) mod stub;

#[cfg(feature = "role_supervisor")]
pub use real::*;
#[cfg(not(feature = "role_supervisor"))]
pub use stub::*;
