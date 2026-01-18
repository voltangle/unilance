use embassy_stm32::Peripherals;
use proto::CoreLink;

#[allow(unused)]
pub fn init_periph(_p: &Peripherals) {}

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
#[allow(unused)]
pub fn start<T: CoreLink>(_link: &T) {}
