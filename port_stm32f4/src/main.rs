#![no_std]
#![no_main]

mod bsp;
mod sthal;

use core::sync::atomic::Ordering;

use embassy_stm32::Peripherals;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_time::Timer;
use mesc::set_motor;
use mesc::MESC_motor_typedef;
use mesc::hw_setup_s;
use mesc::TIM_HandleTypeDef;
use mesc::TIM_TypeDef;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    configure_mesc();

    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    sthal::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    loop {
        info!("Hello World!");
        Timer::after_secs(1).await;
    }
}

/// Can only be called ONCE at firmware init
fn configure_mesc() {
    let mut mtimer: TIM_HandleTypeDef = TIM_HandleTypeDef::default();
    let mut stimer: TIM_HandleTypeDef = TIM_HandleTypeDef::default();
    unsafe {
        mtimer.Instance = bsp::MOTOR_TIM.as_ptr() as *mut TIM_TypeDef;
        stimer.Instance = bsp::MESC_SLOW_LOOP_TIM.as_ptr() as *mut TIM_TypeDef;
    }

    let mut motor = MESC_motor_typedef::default();
    motor.mtimer = &mut mtimer;
    motor.stimer = &mut stimer;

    mesc::set_motor(motor);
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
