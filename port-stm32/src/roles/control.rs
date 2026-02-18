use crate::bsp;
use crate::roles::MemChannelCoreLink;
use core::mem::MaybeUninit;
use core_control::State;
use core_control::balance::BalanceState;
use embassy_executor::Spawner;
use mesc::MESC_motor_typedef;
use proc_macros::for_role;
use static_cell::StaticCell;

// SAFETY: The `balance` field of this struct can ONLY be accessed in an ISR, specifically
// the ISR that runs balance_loop(). Because of this, I opted to not use a mutex,
// for simplicity and performance reasons.
// RESPECT THESE RULES, OR THE CHANCE OF THE WHEEL MAKING EXPENSIVE SOUNDS RISES
// EXPONENTIALLY.
static mut CONTROL_STATE: MaybeUninit<State> = MaybeUninit::uninit();

#[for_role("combined")]
type PlatformCoreLink<'a> = MemChannelCoreLink<'a>;
static CONTROL_CORELINK: StaticCell<PlatformCoreLink> = StaticCell::new();

#[allow(static_mut_refs, unused)]
fn state() -> &'static mut State {
    unsafe { (&mut *CONTROL_STATE.as_mut_ptr()) }
}

#[allow(static_mut_refs)]
pub fn init() {
    // FIXME: redo as new() call
    unsafe {
        CONTROL_STATE.write(State::new());
    }
    // TODO: Try to figure out how to do the hardware config in Rust instead of a C header

    // TODO: This has to be refactored so it uses motor config in BSP
    let motor = MESC_motor_typedef {
        id: 0,
        ..Default::default()
    };

    crate::set_motor(motor);
}

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start(spawner: &Spawner, link: MemChannelCoreLink<'static>) {
    let corelink = CONTROL_CORELINK.init(link);
    spawner
        .spawn(main_task(state(), corelink).expect("failed to start control main task"));
}

/// BALANCE_STATE MUST be initialized when this function runs.
pub fn balance_loop() {
    // TODO: MESC doesn't expose this, make it work later
    // mesc::houseKeeping(mesc::get_motor());

    let motor = crate::get_motor();
    motor.FOC.Idq_req.q = state()
        .balance
        .iterate(core_control::imu::IMUData::default());
}

#[embassy_executor::task]
async fn main_task(
    state: &'static mut State,
    link: &'static mut MemChannelCoreLink<'static>,
) {
    core_control::main_task(state, link).await;
}
