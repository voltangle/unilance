// NOTE: This entire file was written mostly by ChatGPT, because I couldn't be bothered
// to write all this shit myself lmao
use atomic_enum::atomic_enum;
use core::sync::atomic::{AtomicU32, Ordering};
use cortex_m::peripheral::DWT;
use rtos_trace::{RtosTrace, global_trace};

#[inline(always)]
pub fn now_cycles() -> u32 {
    unsafe { (*DWT::PTR).cyccnt.read() }
}

#[atomic_enum]
#[derive(PartialEq)]
enum ExecMode {
    Idle,
    Busy, // includes tasks + ISR
    Isr,  // internal state to avoid overriding while in ISR
}

static MODE: AtomicExecMode = AtomicExecMode::new(ExecMode::Idle);
static LAST_TS: AtomicU32 = AtomicU32::new(0);
static ISR_NEST: AtomicU32 = AtomicU32::new(0);

pub static ACC_IDLE: AtomicU32 = AtomicU32::new(0);
pub static ACC_BUSY: AtomicU32 = AtomicU32::new(0);

/// Call once after enabling DWT CYCCNT (or rely on RtosTrace::start()).
pub fn init() {
    let t = now_cycles();
    LAST_TS.store(t, Ordering::Relaxed);
    MODE.store(ExecMode::Idle, Ordering::Relaxed);
    ISR_NEST.store(0, Ordering::Relaxed);
    ACC_IDLE.store(0, Ordering::Relaxed);
    ACC_BUSY.store(0, Ordering::Relaxed);
}

#[inline(always)]
fn transition_to(new_mode: ExecMode) {
    let t = now_cycles();
    let last = LAST_TS.swap(t, Ordering::Relaxed);
    let dt = t.wrapping_sub(last);

    let prev = MODE.swap(new_mode, Ordering::Relaxed);

    if prev == ExecMode::Idle {
        ACC_IDLE.fetch_add(dt, Ordering::Relaxed);
    } else {
        // Busy or Isr both count as busy
        ACC_BUSY.fetch_add(dt, Ordering::Relaxed);
    }
}

#[derive(Copy, Clone)]
pub struct Snapshot {
    pub idle: u32,
    pub busy: u32,
}

pub fn snapshot() -> Snapshot {
    Snapshot {
        idle: ACC_IDLE.load(Ordering::Relaxed),
        busy: ACC_BUSY.load(Ordering::Relaxed),
    }
}

/// Returns CPU usage percent over the interval [prev, now].
pub fn usage_percent(prev: Snapshot, now: Snapshot) -> f32 {
    let didle = now.idle.wrapping_sub(prev.idle);
    let dbusy = now.busy.wrapping_sub(prev.busy);
    let dtotal = didle.wrapping_add(dbusy);

    if dtotal == 0 {
        0.0
    } else {
        (dbusy as f32) * 100.0 / (dtotal as f32)
    }
}

struct CpuUsageTracer;

impl RtosTrace for CpuUsageTracer {
    fn start() {
        // Prevent "since boot" being charged to idle on first transition.
        init();
    }
    fn stop() {}

    fn task_new(_id: u32) {}
    fn task_new_stackless(_id: u32, _name: &'static str, _priority: u32) {}
    fn task_send_info(_id: u32, _info: rtos_trace::TaskInfo) {}
    fn task_terminate(_id: u32) {}
    fn task_ready_begin(_id: u32) {}
    fn task_ready_end(_id: u32) {}
    fn name_marker(_id: u32, _name: &'static str) {}
    fn marker(_id: u32) {}
    fn marker_begin(_id: u32) {}
    fn marker_end(_id: u32) {}

    // Executor says it's going idle (thread mode)
    fn system_idle() {
        // Don't override ISR state
        if MODE.load(Ordering::Relaxed) != ExecMode::Isr {
            transition_to(ExecMode::Idle);
        }
    }

    // A task begins executing (thread mode)
    fn task_exec_begin(_id: u32) {
        // Don't override ISR state
        if MODE.load(Ordering::Relaxed) != ExecMode::Isr {
            transition_to(ExecMode::Busy);
        }
    }

    fn task_exec_end() {
        // no-op: we stay busy until system_idle() closes it
    }

    fn isr_enter() {
        if ISR_NEST.fetch_add(1, Ordering::Relaxed) == 0 {
            transition_to(ExecMode::Isr);
        }
    }

    fn isr_exit() {
        if ISR_NEST.fetch_sub(1, Ordering::Relaxed) == 1 {
            // After ISR, treat as busy until system_idle() says otherwise
            transition_to(ExecMode::Busy);
        }
    }

    fn isr_exit_to_scheduler() {
        // Conservative: after ISR wakes work, we're busy.
        // (Don’t try to be fancy; system_idle() will later close busy.)
        ISR_NEST.store(0, Ordering::Relaxed);
        transition_to(ExecMode::Busy);
    }
}

global_trace! { CpuUsageTracer }
