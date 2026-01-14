#![no_std]
#![no_main]

mod mesc_bind;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let motor = mesc_bind::MESC_motor_typedef {

    };
    let _p = embassy_stm32::init(config);
    unsafe {
        crate::mesc_bind::MESCfoc_Init(&mut motor);
    }

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}
