use crate::roles::MemChannelCoreLink;
use crate::bsp;
use core::mem::MaybeUninit;
use core_control::State;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::Timer;
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
    spawner.spawn(
        main_task(get_state(), corelink).expect("Controls' main task should start"),
    );
    spawner.spawn(motor_control_view().expect("Motor control view should start"));
}

static mut AUX_OPENLOOP_CNT: u32 = 0;
/// BALANCE_STATE MUST be initialized when this function runs.
#[allow(static_mut_refs)]
pub fn aux_loop() {
    // FIXME: THIS SHOULD NEVER PANIC!!!!!!!!
    // Fix once some kind of error passing system is implemented.
    let imu = bsp::get_imu_data().unwrap();
    let _spacial = get_state().ahrs.update(&imu.0, &imu.1).unwrap();
    // TODO: Reenable when I finish testing motor control
    // get_state()
    //     .motor
    //     .request_q(get_state().balance.update(spacial));
    unsafe {
        if AUX_OPENLOOP_CNT < 1000 {
            AUX_OPENLOOP_CNT += 1;
        }
        get_state()
            .motor
            .request_q(6.0 * (AUX_OPENLOOP_CNT as f32 / 1000.0));
    }
    get_state().motor.foc_aux_update();
}

#[embassy_executor::task]
async fn motor_control_view() {
    loop {
        let m = &get_state().motor;
        info!(
            "RawIu: {}, RawIv: {}, RawIw: {}, Iu: {}, Iv: {}, Iw: {}, Vbus: {}, Iq: {}, key bits: {}, motor state: {}, FOC angle: {}",
            m.Raw.Iu,
            m.Raw.Iv,
            m.Raw.Iw,
            m.Conv.Iu,
            m.Conv.Iv,
            m.Conv.Iw,
            m.Conv.Vbus,
            m.FOC.Idq_prereq.q,
            m.key_bits,
            m.get_state(),
            m.FOC.FOCAngle,
        );
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
async fn main_task(
    state: &'static mut State,
    link: &'static mut MemChannelCoreLink<'static>,
) {
    core_control::main_task(state, link).await;
}

// NOTE: ideally this default init should be in the mesc crate, and be merged into the
// MESC_motor_typedef struct. All of this should NOT be a global.
#[unsafe(export_name = "g_hw_setup")]
pub static mut HW_SETUP: hw_setup_s = hw_setup_s {
    Imax: 20.0,
    Vmax: 170.0,
    Vmin: 0.0,
    Rshunt: 0.0,
    RVBT: 0.0,
    RVBB: 0.0,
    VBGain: 0.0514064227,
    RIphPU: 0.0,
    RIphSR: 0.0,
    OpGain: 0.0,
    // Calculated with this: (ADC_Value - 2048) * ((3.3 / 4095) / gain) = ~350
    // Total current measurement range = 350A
    // Should show 350A as 2,97V on the output, as BSO6920BSO-50A has 1,65V as zero and each
    // amp is each 26,7 mV. 1650mV + (26,7mV * 50) = 2,97V, and 1650mV - (26,7mV * 50) = 0,315V.
    // The sensor itself only sees 50A, as the setup is done with two 3 mΩ shunts in parallel
    // with a 9mΩ hall effect current sensor, with equivalent series resistance of 1,3mΩ.
    // Essentially, the sensor only sees 1/7 of the total current going through the phase its
    // measuring.
    Igain: 0.21127,
    RawCurrLim: 4096,
    RawVoltLim: 4096,
};
