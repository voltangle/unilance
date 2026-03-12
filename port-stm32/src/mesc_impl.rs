use crate::cpu_usage;
use defmt::{debug, error, info, trace, warn};
use embassy_time::Duration;
use mesc::CoreHal;

#[mesc::global_core_hal]
struct MescImpl;

impl CoreHal for MescImpl {
    fn delay_ms(ms: u32) {
        // Blocking delay implemented by Embassy's time driver.
        // This does not require async/await at the call site.
        embassy_time::block_for(Duration::from_millis(ms as u64));
    }

    fn get_cpu_cycles() -> u32 {
        cpu_usage::now_cycles()
    }

    fn log_trace(msg: &str) {
        trace!("{:?}", msg);
    }

    fn log_trace_int(msg: &str, num: u32) {
        trace!("{}{:?}", msg, num);
    }

    fn log_trace_double(msg: &str, num: f64) {
        trace!("{}{:?}", msg, num);
    }

    fn log_debug(msg: &str) {
        debug!("{:?}", msg);
    }

    fn log_info(msg: &str) {
        info!("{:?}", msg);
    }

    fn log_warn(msg: &str) {
        warn!("{:?}", msg);
    }

    fn log_error(msg: &str) {
        error!("{:?}", msg);
    }
}
