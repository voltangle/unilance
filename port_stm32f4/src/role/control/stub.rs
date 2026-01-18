use embassy_stm32::Peripherals;
use proto::CoreLink;

pub fn init_periph(_p: &Peripherals) {}
pub fn start<T: CoreLink>(_link: &T) {}
