mod gestures;

#[cfg(feature = "control_scheme_power_aux")]
mod power_aux;
#[cfg(feature = "control_scheme_power_aux")]
pub use power_aux::*;

#[allow(unused)]
pub trait InputMethods {
    fn is_pressed(role: ButtonRole) -> bool;
    /// Returns how far did the dial (based on a relative encoder) move relative to when
    /// this was called last time.
    fn dial_relative_distance() -> i16;
    /// Returns the dials' absolute position. Only for absolute encoders/potentiometers.
    fn dial_absolute_position() -> i32;
}

#[allow(unused)]
unsafe extern "Rust" {
    pub fn unilance_input_is_pressed(role: ButtonRole) -> bool;
    pub fn unilance_input_dial_relative_distance() -> i16;
    pub fn unilance_input_dial_absolute_position() -> i32;
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
