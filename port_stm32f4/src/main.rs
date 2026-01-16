#![no_std]
#![no_main]

mod sthal_embassy;
mod bsp;

use core::sync::atomic::Ordering;

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use unilance_mesc::c_bind::MESC_motor_typedef;
use unilance_mesc::c_bind::hw_setup_s;
use {defmt_rtt as _, panic_probe as _};

// NOTE: ideally this default init should be in the mesc crate
#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
pub static mut g_hw_setup: hw_setup_s = hw_setup_s {
    Imax: 0.0,
    Vmax: 0.0,
    Vmin: 0.0,
    Rshunt: 0.0,
    RVBT: 0.0,
    RVBB: 0.0,
    VBGain: 0.0,
    RIphPU: 0.0,
    RIphSR: 0.0,
    OpGain: 0.0,
    Igain: 0.0,
    RawCurrLim: 0,
    RawVoltLim: 0
};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    sthal_embassy::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    let mut motor = MESC_motor_typedef::default();
    unsafe {
        unilance_mesc::c_bind::MESCfoc_Init(&mut motor);
    }

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}

// MESC hooks

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_1(motor: &mut MESC_motor_typedef) {
    unsafe { bsp::init_1(motor); }
}

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_2(motor: &mut MESC_motor_typedef) {
    unsafe { bsp::init_2(motor); }
}

#[unsafe(no_mangle)]
pub extern "C" fn mesc_init_3(motor: &mut MESC_motor_typedef) {
    unsafe { bsp::init_3(motor); }
}

#[unsafe(no_mangle)]
pub extern "C" fn hw_init(motor: &mut MESC_motor_typedef) {
    unsafe { bsp::hw_init(motor); }
}
