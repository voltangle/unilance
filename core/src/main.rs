#![no_std]
#![no_main]

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
    let _p = embassy_stm32::init(config);

    let mut motor = MESC_motor_typedef::default();
    mesc_start(&mut motor);

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}
