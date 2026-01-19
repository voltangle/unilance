use super::PlatformConfig;
use embassy_stm32::gpio;
use embassy_stm32::interrupt;
use embassy_stm32::pac::timer::{TimAdv, TimGp16};
use embassy_stm32::pac::{ADC1, ADC2, ADC3, GPIOB};
use embassy_stm32::rcc::{Hse, HseMode};
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::complementary_pwm::ComplementaryPwm;
use embassy_stm32::{Config, Peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use mesc::MESC_ADC_IRQ_handler;
use mesc::MESC_PWM_IRQ_handler;
use mesc::MESC_motor_typedef;
use mesc::hw_setup_s;
use proc_macros::for_role;
use static_cell::StaticCell;

/*
 * MESC OWNED PERIPHERALS:
 * - TIM8
 * - PB6, PB7, PB8
 * - ADC1, ADC2, ADC3
 * These are NOT to be interacted with from Embassy, IT WILL ALMOST DEFINITELY RESULT
 * IN THE WHEEL MAKING EXPENSIVE SOUNDS
 */

// TODO: Figure out how to do "input methods". Some wheels will have controls like Begode,
// where there is just a power button and a park button, some wheels will have something
// like LeaperKim, with power + headlight + OK + next, etc etc. I have to figure out
// how to make it all coexist

pub const STARTUP_DELAY_MS: u64 = 1500;

pub struct BspPeripherals<'a> {
    poweron: gpio::Output<'a>,
    power_button: gpio::Input<'a>,
    park_button: gpio::Input<'a>,
}

// Gather all peripherals required for opereration and initialize anything that
// needs to be initialized at this point. This function has to be called ONCE on boot.
pub fn init<'a>(p: Peripherals) -> BspPeripherals<'a> {
    BspPeripherals {
        poweron: gpio::Output::new(p.PB14, gpio::Level::Low, gpio::Speed::Medium),
        power_button: gpio::Input::new(p.PB15, gpio::Pull::Down),
        park_button: gpio::Input::new(p.PA12, gpio::Pull::Down),
    }
}

/*
 * MESC configuration
 */

// Peripheral table
pub const MESC_MOTOR_TIM: TimAdv = embassy_stm32::pac::TIM8;
pub const MESC_SLOW_LOOP_TIM: TimGp16 = embassy_stm32::pac::TIM3;

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

pub fn init_1(_motor: &mut MESC_motor_typedef) {}

pub fn init_2(_motor: &mut MESC_motor_typedef) {}

pub fn init_3(_motor: &mut MESC_motor_typedef) {
    unsafe {
        let _p = Peripherals::steal();
        // let mut tim = ComplementaryPwm::new(p.TIM8);

        // TODO: Implement
    }
}

pub fn hw_init(_motor: &mut MESC_motor_typedef) {
    unimplemented!()
}

// PB6, 7, 8
pub fn get_hall_state() -> u8 {
    ((GPIOB.idr().read().0 >> 6) & 0b111) as u8
}

/*
 * Clock configurations
 */

// NOTE: No idea if it actually makes sense to do an extension trait, but shit, it looks
// nice when used

impl PlatformConfig for Config {
    fn for_platform() -> Self {
        let mut config = Config::default();

        config.rcc.hsi = false;
        config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(8),
            mode: HseMode::Bypass,
        });
        config
    }
}

/*
 * Interrupts
 */

#[interrupt]
fn TIM8_UP_TIM13() {
    unsafe {
        MESC_PWM_IRQ_handler(mesc::get_motor());
        // Clear update flag
        MESC_MOTOR_TIM.sr().modify(|w| w.set_uif(false));
    }
}

#[interrupt]
fn ADC() {
    unsafe {
        MESC_ADC_IRQ_handler(mesc::get_motor());
        // FIXME: ADC flags are NOT BEING RESET, IT WILL NOT FIRE AGAIN
    }
}

/*
 * Platform functions
 */

pub fn startup_successful(periph: &mut BspPeripherals) {
    periph.poweron.set_high();
}

pub fn refresh_adc() {
    unimplemented!()
}

pub fn refresh_adc_for_vphase() {
    unimplemented!()
}
