#![no_std]
#![no_main]

mod constants;
mod cpu_usage;
mod driver;
mod mesc_impl;
mod roles;
mod tsp;

#[for_role("combined")]
use crate::roles::{CoreChannel, MemChannelCoreLink};
use crate::tsp::PlatformConfig;
use core::ptr::read_volatile;
use core::sync::atomic::Ordering;
use cortex_m::Peripherals;
use cortex_m_rt::{ExceptionFrame, exception};
use defmt::{error, info};
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
    let p = embassy_stm32::init(Config::for_platform());
    let clocks = embassy_stm32::rcc::clocks(&p.RCC);
    mesc_impl::HCLK_HZ.store(clocks.hclk1.to_hertz().unwrap().0, Ordering::Relaxed);

    let startup_timer = Timer::after_millis(tsp::STARTUP_DELAY_MS);
    Timer::after_millis(2000).await;

    tsp::init(p, &spawner).await;
    info!("BSP init finished");
    #[cfg(feature = "role_supervisor")]
    roles::supervisor::init();
    #[cfg(feature = "role_control")]
    roles::control::init();
    info!("Roles initialized");

    #[cfg(feature = "role_supervisor")]
    roles::supervisor::start(&spawner, make_core_link(true));
    #[cfg(feature = "role_control")]
    roles::control::start(&spawner, make_core_link(false));
    info!("Roles started");

    // the timer starts "counting" right after it was created (it just saves a timestamp of
    // when it's supposed to elapse), so .await will "let go" exactly after STARTUP_DELAY_MS
    startup_timer.await;
    tsp::startup_successful();
    info!("Hello from UniLANCE!");

    unsafe {
        let mut p = Peripherals::steal();
        p.DCB.enable_trace();
        p.DWT.enable_cycle_counter();
        cpu_usage::init();
    }

    let mut prev = cpu_usage::snapshot();
    loop {
        Timer::after_secs(1).await;
        let now = cpu_usage::snapshot();
        let usage = cpu_usage::usage_percent(prev, now);
        info!("CPU usage: {}%", usage);
        prev = now;
    }

    // Park indefinitely, so all other tasks can just, uhh, run
    // core::future::pending::<()>().await;
    // unreachable!();
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

#[exception]
unsafe fn HardFault(frame: &ExceptionFrame) -> ! {
    unsafe {
        const CFSR: *mut u32 = 0xE000ED28 as *mut u32;
        error!(
            "HardFault triggered! xpsr: {:#010x}, cfsr: {:#010x}",
            frame.xpsr(),
            read_volatile(CFSR)
        );
        loop {}
    }
}
