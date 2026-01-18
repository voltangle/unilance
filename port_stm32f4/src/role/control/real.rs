use crate::sthal;
use core::sync::atomic::Ordering;
use embassy_stm32::Peripherals;
use proto::CoreLink;

pub fn init_periph(p: &Peripherals) {
    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    sthal::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);
}

/// Start all supervisor stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start<T: CoreLink>(_link: &T) {}
