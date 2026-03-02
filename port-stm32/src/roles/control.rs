use crate::bsp;
use crate::roles::MemChannelCoreLink;
use core::mem::MaybeUninit;
use core_control::State;
use embassy_executor::Spawner;
use mesc::{MescMotorExt, hw_setup_s};
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

/// Returns a mutable reference to the static State instance.
#[allow(static_mut_refs, unused)]
pub fn get_state() -> &'static mut State {
    unsafe { (&mut *CONTROL_STATE.as_mut_ptr()) }
}

#[allow(static_mut_refs)]
pub fn init() {
    unsafe {
        CONTROL_STATE.write(State::new());
        get_state().motor.init();
    }
}

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start(spawner: &Spawner, link: MemChannelCoreLink<'static>) {
    let corelink = CONTROL_CORELINK.init(link);
    spawner.spawn({
        match main_task(get_state(), corelink) {
            Ok(result) => result,
            Err(_) => {
                panic!("Failed to initialize control main task");
            }
        }
    });
}

/// BALANCE_STATE MUST be initialized when this function runs.
pub fn aux_loop() {
    get_state().motor.foc_aux_update();
    // FIXME: THIS SHOULD NEVER PANIC!!!!!!!!
    // Fix once some kind of error passing system is implemented.
    let imu = bsp::get_imu_data().unwrap();
    let spacial = get_state().ahrs.update(&imu.0, &imu.1).unwrap();
    get_state()
        .motor
        .request_q(get_state().balance.update(spacial));
}

pub fn motor_loop() {
    core_control::pwm_isr(get_state());
}

#[embassy_executor::task]
async fn main_task(
    state: &'static mut State,
    link: &'static mut MemChannelCoreLink<'static>,
) {
    core_control::main_task(state, link).await;
}

// NOTE: ideally this default init should be in the mesc crate
#[unsafe(export_name = "g_hw_setup")]
pub static mut HW_SETUP: hw_setup_s = hw_setup_s {
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
    RawVoltLim: 0,
};
