use embassy_stm32::Peripherals;
use proto::CoreLink;

#[allow(unused)]
pub fn start<T: CoreLink>(_link: &T) {}
#[allow(unused)]
pub fn init() {}
#[allow(unused)]
pub fn balance_loop() {}
