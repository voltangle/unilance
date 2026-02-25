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
use core::sync::atomic::Ordering;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{Config, bind_interrupts};
#[for_role("combined")]
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use proc_macros::for_role;

use panic_probe as _;

static CTRL_TO_SUPV_CHANNEL: CoreChannel = Channel::new();
static SUPV_TO_CTRL_CHANNEL: CoreChannel = Channel::new();

bind_interrupts!(struct Irqs {
    USART1 => embassy_stm32::usart::InterruptHandler<embassy_stm32::peripherals::USART1>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    // let p = embassy_stm32::init(Config::default());
    let p = embassy_stm32::init(Config::for_platform());
    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    mesc_impl::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    let startup_timer = Timer::after_millis(bsp::STARTUP_DELAY_MS);
    Timer::after_millis(2000).await;

    bsp::init(p, &spawner);
    info!("BSP init finished");
    #[cfg(feature = "role_supervisor")]
    {
        roles::supervisor::init();
        info!("Supervisor init finished");
    }
    #[cfg(feature = "role_control")]
    {
        roles::control::init();
        info!("Control init finished");
    }
    info!("Roles initialized");

    #[cfg(feature = "role_supervisor")]
    {
        roles::supervisor::start(&spawner, make_core_link(true));
        info!("Supervisor started");
    }
    #[cfg(feature = "role_control")]
    {
        roles::control::start(&spawner, make_core_link(false));
        info!("Control started");
    }
    info!("Roles started");

    // the timer starts "counting" right after it was created (it just saves a timestamp of
    // when it's supposed to elapse), so .await will "let go" exactly after STARTUP_DELAY_MS
    startup_timer.await;
    bsp::startup_successful();
    info!("Hello from UniLANCE!");

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
