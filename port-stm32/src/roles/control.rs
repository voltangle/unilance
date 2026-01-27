use crate::bsp;
use core::mem::MaybeUninit;
use core_control::balance::BalanceState;
use mesc::MESC_motor_typedef;
use proto::CoreLink;

// ACCESS RULES: This struct can ONLY be accessed in an ISR, specifically the
// ISR that runs balance_loop(). Because of this, I opted to not use a mutex,
// for simplicity and performance reasons.
// RESPECT THESE RULES, OR THE CHANCE OF THE WHEEL MAKING EXPENSIVE SOUNDS RISES
// EXPONENTIALLY.
static mut BALANCE_STATE: MaybeUninit<BalanceState> = MaybeUninit::uninit();

#[allow(static_mut_refs, unused)]
fn balance_state() -> &'static mut BalanceState {
    unsafe { (&mut *BALANCE_STATE.as_mut_ptr()) }
}

#[allow(static_mut_refs)]
pub fn init() {
    let mut balance_state = BalanceState::default();
    balance_state.config = bsp::BALANCE_CONF;
    balance_state.init();
    unsafe {
        BALANCE_STATE.write(balance_state);
    }
    // TODO: Try to figure out how to do the hardware config in Rust instead of a C header

    // TODO: This has to be refactored so it uses motor config in BSP
    let motor = MESC_motor_typedef { id: 0, ..Default::default() };

    crate::set_motor(motor);
}

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start<T: CoreLink>(_link: &T) {}

/// BALANCE_STATE MUST be initialized when this function runs.
pub fn balance_loop() {
    // TODO: MESC doesn't expose this, make it work later
    // mesc::houseKeeping(mesc::get_motor());

    let motor = crate::get_motor();
    motor.FOC.Idq_req.q = balance_state().iterate(core_control::ahrs::IMUData::default());
}
