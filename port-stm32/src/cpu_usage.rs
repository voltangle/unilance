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
    Busy,
    ISR,
}

static MODE: AtomicExecMode = AtomicExecMode::new(ExecMode::Idle);
static LAST_TS: AtomicU32 = AtomicU32::new(0);
static ISR_NEST: AtomicU32 = AtomicU32::new(0);

static ACC_IDLE: AtomicU32 = AtomicU32::new(0);
static ACC_BUSY: AtomicU32 = AtomicU32::new(0);

#[inline(always)]
fn transition_to(new_mode: ExecMode) {
    let t = now_cycles();
    let last = LAST_TS.swap(t, Ordering::Relaxed);

    // dt in cycles (wrap-safe)
    let dt = t.wrapping_sub(last);

    let prev = MODE.swap(new_mode, Ordering::Relaxed);
    if prev == ExecMode::Idle {
        ACC_IDLE.fetch_add(dt, Ordering::Relaxed);
    } else {
        ACC_BUSY.fetch_add(dt, Ordering::Relaxed);
    }
}

struct CpuUsageTracer {}

impl RtosTrace for CpuUsageTracer {
    fn start() {}
    fn stop() {}

    fn task_new(_id: u32) {}
    fn task_send_info(_id: u32, _info: rtos_trace::TaskInfo) {}
    fn task_new_stackless(_id: u32, _name: &'static str, _priority: u32) {}
    fn task_terminate(_id: u32) {}
    fn task_ready_begin(_id: u32) {}
    fn task_ready_end(_id: u32) {}
    fn name_marker(_id: u32, _name: &'static str) {}
    fn marker(_id: u32) {}
    fn marker_begin(_id: u32) {}
    fn marker_end(_id: u32) {}

    // Embassy tells us it is going idle.
    fn system_idle() {
        transition_to(ExecMode::Idle);
    }

    // Task starts executing in thread-mode.
    fn task_exec_begin(_id: u32) {
        // If we were idle, this will close the idle interval.
        // If we were ISR, we stay ISR until isr_exit.
        if MODE.load(Ordering::Relaxed) != ExecMode::ISR {
            transition_to(ExecMode::Busy);
        }
    }

    fn task_exec_end() {
        // Not strictly needed for utilization; leave as no-op or keep BUSY.
        // Keeping as BUSY is fine; the next system_idle() will close the busy interval.
    }

    fn isr_enter() {
        // Handle nesting: only transition on first entry.
        if ISR_NEST.fetch_add(1, Ordering::Relaxed) == 0 {
            transition_to(ExecMode::ISR);
        }
    }

    fn isr_exit() {
        // Only transition back when exiting the outermost ISR.
        if ISR_NEST.fetch_sub(1, Ordering::Relaxed) == 1 {
            // After ISR, we assume we're busy until the executor declares idle again.
            transition_to(ExecMode::Busy);
        }
    }

    fn isr_exit_to_scheduler() {
        // SAFETY: treat as busy.
        // (Some systems use this when an ISR wakes the scheduler.)
        // Using BUSY is fine; system_idle() will later switch to IDLE.
        // If nesting bookkeeping is correct, this is rarely needed.
        transition_to(ExecMode::Busy);
    }
}

global_trace! { CpuUsageTracer }
