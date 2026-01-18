use embassy_stm32::Peripherals;
use proto::CoreLink;

#[allow(unused)]
pub fn init_periph(_p: &Peripherals) {}
#[allow(unused)]
pub fn start<T: CoreLink>(_link: &T) {}
