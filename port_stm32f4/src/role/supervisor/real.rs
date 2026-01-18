use embassy_stm32::Peripherals;
use proto::CoreLink;

pub fn init_periph(_p: &Peripherals) {}

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start<T: CoreLink>(_link: &T) {}
