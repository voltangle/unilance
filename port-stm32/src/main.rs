#![no_std]
#![no_main]

mod bsp;
mod constants;
mod cpu_usage;
mod driver;
mod mesc_impl;
mod roles;

use crate::bsp::PlatformConfig;
#[for_role("combined")]
use crate::roles::{CoreChannel, MemChannelCoreLink};
use core::mem::MaybeUninit;
use core::sync::atomic::Ordering;
use embassy_executor::Spawner;
use embassy_stm32::Config;
#[for_role("combined")]
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use proc_macros::for_role;

use {defmt_rtt as _, panic_probe as _};

static CTRL_TO_SUPV_CHANNEL: CoreChannel = Channel::new();
static SUPV_TO_CTRL_CHANNEL: CoreChannel = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Config::for_platform());
    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    mesc_impl::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    let startup_timer = Timer::after_millis(bsp::STARTUP_DELAY_MS);

    bsp::init(p, &spawner);
    #[cfg(feature = "role_supervisor")]
    roles::supervisor::init();
    #[cfg(feature = "role_control")]
    roles::control::init();

    #[cfg(feature = "role_supervisor")]
    roles::supervisor::start(&spawner, make_core_link(true));
    #[cfg(feature = "role_control")]
    roles::control::start(&spawner, make_core_link(false));

    // the timer starts "counting" right after it was created (it just saves a timestamp of
    // when it's supposed to elapse), so .await will "let go" exactly after STARTUP_DELAY_MS
    startup_timer.await;
    bsp::startup_successful();

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
