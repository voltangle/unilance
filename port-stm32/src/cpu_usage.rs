use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

use cortex_m::peripheral::DWT;
use rtos_trace::{global_trace, RtosTrace};

// ===== Timestamp source =====
// Prefer DWT CYCCNT when available; otherwise use a free-running hardware timer.
// This function must be very fast and callable in ISR context.
#[inline(always)]
fn now_cycles() -> u32 {
    // DWT CYCCNT example (Cortex-M3/M4/M7). Ensure you've enabled it at boot.
    unsafe { (*DWT::ptr()).cyccnt.read() }
}

// Modes
const MODE_IDLE: u8 = 0;
const MODE_BUSY: u8 = 1;
const MODE_ISR:  u8 = 2;

// State
static MODE: AtomicU8 = AtomicU8::new(MODE_BUSY);
static LAST_TS: AtomicU32 = AtomicU32::new(0);
static ISR_NEST: AtomicU32 = AtomicU32::new(0);

// Accumulators (32-bit only)
static ACC_IDLE: AtomicU32 = AtomicU32::new(0);
static ACC_BUSY: AtomicU32 = AtomicU32::new(0);

#[inline(always)]
fn transition_to(new_mode: u8) {
    let t = now_cycles();
    let last = LAST_TS.swap(t, Ordering::Relaxed);

    // dt in cycles (wrap-safe)
    let dt = t.wrapping_sub(last);

    let prev = MODE.swap(new_mode, Ordering::Relaxed);
    if prev == MODE_IDLE {
        ACC_IDLE.fetch_add(dt, Ordering::Relaxed);
    } else {
        // BUSY and ISR both count as busy
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
        transition_to(MODE_IDLE);
    }

    // Task starts executing in thread-mode.
    fn task_exec_begin(_id: u32) {
        // If we were idle, this will close the idle interval.
        // If we were ISR, we stay ISR until isr_exit.
        if MODE.load(Ordering::Relaxed) != MODE_ISR {
            transition_to(MODE_BUSY);
        }
    }

    fn task_exec_end() {
        // Not strictly needed for utilization; leave as no-op or keep BUSY.
        // Keeping as BUSY is fine; the next system_idle() will close the busy interval.
    }

    fn isr_enter() {
        // Handle nesting: only transition on first entry.
        if ISR_NEST.fetch_add(1, Ordering::Relaxed) == 0 {
            transition_to(MODE_ISR);
        }
    }

    fn isr_exit() {
        // Only transition back when exiting the outermost ISR.
        if ISR_NEST.fetch_sub(1, Ordering::Relaxed) == 1 {
            // After ISR, we assume we're busy until the executor declares idle again.
            transition_to(MODE_BUSY);
        }
    }

    fn isr_exit_to_scheduler() {
        // Conservative: treat as busy.
        // (Some systems use this when an ISR wakes the scheduler.)
        // Using BUSY is fine; system_idle() will later switch to IDLE.
        // If nesting bookkeeping is correct, this is rarely needed.
        transition_to(MODE_BUSY);
    }
}

global_trace! { CpuUsageTracer }
