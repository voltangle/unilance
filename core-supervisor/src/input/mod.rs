mod gestures;

#[cfg(feature = "control_scheme_power_aux")]
mod power_aux;
#[cfg(feature = "control_scheme_power_aux")]
pub use power_aux::*;

#[allow(unused)]
pub trait InputMethods {
    fn is_pressed(role: ButtonRole) -> bool;
}

#[allow(unused)]
unsafe extern "Rust" {
    pub(crate) fn unilance_input_is_pressed(role: ButtonRole) -> bool;
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum ButtonRole {
    Power,
    Aux,
    Next,
    Headlight,
    Ok,
}
