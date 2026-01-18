#![no_std]
#![no_main]

mod bsp;
mod role;
mod sthal;

#[cfg(any(feature = "role_control", feature = "role_supervisor"))]
use crate::role::{CoreChannel, MemChannelCoreLink};
use embassy_executor::Spawner;
use embassy_stm32::Config;
#[cfg(all(feature = "role_control", feature = "role_supervisor"))]
use embassy_sync::channel::Channel;
use mesc::MESC_motor_typedef;
use mesc::TIM_HandleTypeDef;
use mesc::TIM_TypeDef;

use {defmt_rtt as _, panic_probe as _};

static CTRL_TO_SUPV_CHANNEL: CoreChannel = Channel::new();
static SUPV_TO_CTRL_CHANNEL: CoreChannel = Channel::new();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let supervisor_core_link = make_core_link(true);
    let control_core_link = make_core_link(false);

    /*
     * Supervisor init
     */

    role::supervisor::init_periph(&p);
    role::supervisor::start(&supervisor_core_link);

    /*
     * Control init
     */

    role::control::init_periph(&p);
    role::control::start(&control_core_link);

    // Park indefinitely, so all other tasks can just, uhh, run
    core::future::pending::<()>().await;
    unreachable!();
}

#[cfg(all(feature = "role_control", feature = "role_supervisor"))]
fn make_core_link(is_for_supervisor: bool) -> MemChannelCoreLink<'static> {
    if is_for_supervisor {
        MemChannelCoreLink::new(&SUPV_TO_CTRL_CHANNEL, &CTRL_TO_SUPV_CHANNEL)
    } else {
        MemChannelCoreLink::new(&CTRL_TO_SUPV_CHANNEL, &SUPV_TO_CTRL_CHANNEL)
    }
}

#[cfg(all(
    any(feature = "role_control", feature = "role_supervisor"),
    not(all(feature = "role_control", feature = "role_supervisor"))
))]
fn make_core_link(is_for_supervisor: bool) -> CanBusCoreLink {
    unimplemented!()
}

/// Can only be called ONCE at firmware init
fn configure_mesc() {
    let mut mtimer: TIM_HandleTypeDef = TIM_HandleTypeDef::default();
    let mut stimer: TIM_HandleTypeDef = TIM_HandleTypeDef::default();

    mtimer.Instance = bsp::MOTOR_TIM.as_ptr() as *mut TIM_TypeDef;
    stimer.Instance = bsp::MESC_SLOW_LOOP_TIM.as_ptr() as *mut TIM_TypeDef;

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
