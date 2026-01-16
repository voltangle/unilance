#![no_std]
#![no_main]

mod sthal_embassy;

use core::sync::atomic::Ordering;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use unilance_mesc::c_bind::MESC_motor_typedef;
use unilance_mesc::mesc_start;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    // This is the step you said requires p.RCC:
    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    sthal_embassy::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    let mut motor = MESC_motor_typedef::default();
    mesc_start(&mut motor);

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}
