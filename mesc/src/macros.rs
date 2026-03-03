/// A macro for binding a MESC HAL implementation to MESC itself. Only binds functions
/// that differ between motor implementations; functions for "core" stuff (delay, log, etc)
/// are set using [global_mesc_core_hal!].
#[macro_export]
macro_rules! global_hal {
    ($ident:ident) => {
        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESC_getHallState() -> i32 {
            <$ident as $crate::Hal>::get_hall_state() as i32
        }

        // TODO: Make the naming convention correct

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESCfoc_getRawADC() {
            <$ident as $crate::Hal>::refresh_adc();
        }

        #[allow(non_snake_case)]
        #[unsafe(no_mangle)]
        extern "C" fn MESCfoc_getRawADCVph() {
            <$ident as $crate::Hal>::refresh_adc_for_vphase();
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_setDeadtimeNs(motor: &mut MESC_motor_typedef, ns: u16) {
            <$ident as $crate::Hal>::set_deadtime(motor, ns);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_break(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::phase_a_break(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_break(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::phase_b_break(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_break(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::phase_c_break(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_enable(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::phase_a_enable(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_enable(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::phase_b_enable(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_enable(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::phase_c_enable(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_enableOutput(motor: &mut MESC_motor_typedef) {
            <$ident as $crate::Hal>::enable_output(motor);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
            <$ident as $crate::Hal>::phase_a_set_duty(motor, duty);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
            <$ident as $crate::Hal>::phase_b_set_duty(motor, duty);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
            <$ident as $crate::Hal>::phase_c_set_duty(motor, duty);
        }

        // TODO: This is something that is not supposed to happen.
        // This is only here so for one usecase the timer continues generating update interrupts.
        // Make it work differently
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phD_setDuty(motor: &mut MESC_motor_typedef, duty: u16) {
            <$ident as $crate::Hal>::phase_d_set_duty(motor, duty);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phA_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
            <$ident as $crate::Hal>::phase_a_get_duty(motor)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phB_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
            <$ident as $crate::Hal>::phase_b_get_duty(motor)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_phC_getDuty(motor: &mut MESC_motor_typedef) -> u16 {
            <$ident as $crate::Hal>::phase_c_get_duty(motor)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_getMaxDuty(motor: &mut MESC_motor_typedef) -> u16 {
            <$ident as $crate::Hal>::get_max_duty(motor)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_setPWMFrequency(motor: &mut MESC_motor_typedef, freq: u32) {
            <$ident as $crate::Hal>::set_pwm_frequency(motor, freq);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_setIRQ(motor: &mut MESC_motor_typedef, state: bool) {
            <$ident as $crate::Hal>::set_irq(motor, state);
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_isTimerCountingDown(
            motor: &mut MESC_motor_typedef,
        ) -> bool {
            <$ident as $crate::Hal>::is_tim_counting_down(motor)
        }
    };
}

#[macro_export]
macro_rules! global_core_hal {
    ($ident:ident) => {
        #[unsafe(no_mangle)]
        extern "C" fn MESChal_delayMs(ms: u32) {
            <$ident as $crate::CoreHal>::delay_ms(ms)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_getTimerHz() -> u32 {
            <$ident as $crate::CoreHal>::get_timer_hz()
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_getCPUCycles() -> u32 {
            <$ident as $crate::CoreHal>::get_cpu_cycles()
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logTrace(string: *const c_char) {
            <$ident as $crate::CoreHal>::log_trace(unsafe {
                CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logTraceDouble(msg: *const c_char, num: c_double) {
            <$ident as $crate::CoreHal>::log_trace_double(unsafe {
                CStr::from_ptr(msg).to_str().unwrap()
            }, num as f64)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logTraceInt(msg: *const c_char, num: c_uint) {
            <$ident as $crate::CoreHal>::log_trace_int(unsafe {
                CStr::from_ptr(msg).to_str().unwrap()
            }, num as u32)
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logDebug(string: *const c_char) {
            <$ident as $crate::CoreHal>::log_debug(unsafe {
                CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logInfo(string: *const c_char) {
            <$ident as $crate::CoreHal>::log_info(unsafe {
                CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logWarn(string: *const c_char) {
            <$ident as $crate::CoreHal>::log_warn(unsafe {
                CStr::from_ptr(string).to_str().unwrap()
            })
        }

        #[unsafe(no_mangle)]
        extern "C" fn MESChal_logError(string: *const c_char) {
            <$ident as $crate::CoreHal>::log_error(unsafe {
                CStr::from_ptr(string).to_str().unwrap()
            })
        }
    };
}
