#![no_std]
#![no_main]

mod bsp;
mod sthal_embassy;

use core::sync::atomic::Ordering;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use mesc::MESC_motor_typedef;
use mesc::hw_setup_s;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    sthal_embassy::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    let mut motor = MESC_motor_typedef::default();
    unsafe {
        mesc::MESCfoc_Init(&mut motor);
    }

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}

// MESC hooks

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_1(motor: &mut MESC_motor_typedef) {
    unsafe {
        bsp::init_1(motor);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_2(motor: &mut MESC_motor_typedef) {
    unsafe {
        bsp::init_2(motor);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_3(motor: &mut MESC_motor_typedef) {
    unsafe {
        bsp::init_3(motor);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn hw_init(motor: &mut MESC_motor_typedef) {
    unsafe {
        bsp::hw_init(motor);
    }
}
