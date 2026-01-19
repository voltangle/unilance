#![no_std]
#![no_main]

mod bsp;
mod reg_flags;
mod roles;
mod sthal;

use crate::bsp::PlatformConfig;
#[for_role("combined")]
use crate::roles::{CoreChannel, MemChannelCoreLink};
use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::rcc::Hse;
use embassy_stm32::rcc::HseMode;
use embassy_stm32::time::Hertz;
#[for_role("combined")]
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use mesc::MESC_motor_typedef;
use mesc::TIM_HandleTypeDef;
use mesc::TIM_TypeDef;
use proc_macros::for_role;

use {defmt_rtt as _, panic_probe as _};

static CTRL_TO_SUPV_CHANNEL: CoreChannel = Channel::new();
static SUPV_TO_CTRL_CHANNEL: CoreChannel = Channel::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Config::for_platform());
    sthal::init(&p);
    let startup_timer = Timer::after_millis(bsp::STARTUP_DELAY_MS);

    let mut bsp_periph = bsp::init(p);

    let supervisor_core_link = make_core_link(true);
    let control_core_link = make_core_link(false);
    roles::supervisor::start(&supervisor_core_link);
    roles::control::start(&control_core_link);

    startup_timer.await;
    // the timer starts "counting" right after it was created (it just saves a timestamp of
    // when it's supposed to elapse), so .await will "let go" exactly after 2 seconds
    bsp::startup_successful(&mut bsp_periph);

    // Park indefinitely, so all other tasks can just, uhh, run
    core::future::pending::<()>().await;
    unreachable!();
}

#[for_role("combined")]
fn make_core_link(is_for_supervisor: bool) -> MemChannelCoreLink<'static> {
    if is_for_supervisor {
        MemChannelCoreLink::new(&SUPV_TO_CTRL_CHANNEL, &CTRL_TO_SUPV_CHANNEL)
    } else {
        MemChannelCoreLink::new(&CTRL_TO_SUPV_CHANNEL, &SUPV_TO_CTRL_CHANNEL)
    }
}

#[for_role("either")]
fn make_core_link(is_for_supervisor: bool) -> CanBusCoreLink {
    unimplemented!()
}

/// Can only be called ONCE at firmware init
fn configure_mesc() {
    let mut mtimer: TIM_HandleTypeDef = TIM_HandleTypeDef::default();
    let mut stimer: TIM_HandleTypeDef = TIM_HandleTypeDef::default();

    mtimer.Instance = bsp::MESC_MOTOR_TIM.as_ptr() as *mut TIM_TypeDef;
    stimer.Instance = bsp::MESC_SLOW_LOOP_TIM.as_ptr() as *mut TIM_TypeDef;

    let mut motor = MESC_motor_typedef::default();
    motor.mtimer = &mut mtimer;
    motor.stimer = &mut stimer;

    mesc::set_motor(motor);
}
