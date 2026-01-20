#[cfg(feature = "board_begode_etmax")]
mod begode_etmax;

#[cfg(feature = "board_begode_etmax")]
pub use begode_etmax::*;

pub trait PlatformConfig {
    fn for_platform() -> Self;
}

// MESC hooks

mod mesc_hooks {
    use mesc::MESC_motor_typedef;

    #[unsafe(no_mangle)]
    pub extern "C" fn mesc_init_1(motor: &mut MESC_motor_typedef) {
        super::init_1(motor);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn mesc_init_2(motor: &mut MESC_motor_typedef) {
        super::init_2(motor);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn mesc_init_3(motor: &mut MESC_motor_typedef) {
        super::init_3(motor);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn hw_init(motor: &mut MESC_motor_typedef) {
        super::hw_init(motor);
    }

    #[allow(non_snake_case)]
    #[unsafe(no_mangle)]
    pub extern "C" fn MESC_getHallState() -> i32 {
        super::get_hall_state() as i32
    }

    #[allow(non_snake_case)]
    #[unsafe(no_mangle)]
    extern "C" fn getRawADC() {
        super::refresh_adc();
    }

    #[allow(non_snake_case)]
    #[unsafe(no_mangle)]
    extern "C" fn getRawADCVph() {
        super::refresh_adc_for_vphase();
    }
}
