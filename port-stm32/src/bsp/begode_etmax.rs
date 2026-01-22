use super::PlatformConfig;
use crate::roles;
use core::mem::MaybeUninit;
use core_control::balance::BalanceConfig;
use core_control::balance::RideAssistConfig;
use embassy_stm32::adc;
use embassy_stm32::adc::Adc;
use embassy_stm32::adc::AdcChannel;
use embassy_stm32::adc::AnyAdcChannel;
use embassy_stm32::adc::RegularConversionMode;
use embassy_stm32::adc::RingBufferedAdc;
use embassy_stm32::adc::SampleTime;
use embassy_stm32::gpio;
use embassy_stm32::interrupt;
use embassy_stm32::pac::timer::{TimAdv, TimGp16};
use embassy_stm32::pac::{ADC1, ADC2, ADC3, GPIOB};
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::peripherals::ADC2;
use embassy_stm32::peripherals::ADC3;
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
 * BSP for the Begode ET Max electric unicycle motherboard.
 *
 * MCU: STM32F405RG
 *
 * Peripherals used in this BSP:
 * - TIM8 on PC6,7,8, PA7, PB0,1: motor control
 * - TIM3 on PA6: Tail light WS281x
 * - TIM4 on PB9: Active(?) buzzer
 * - SPI1 on PB3,4,5 + SPI1 SS PA15: MPU6500 IMU
 * - USART1 on PA9,10: BLE module
 * - USART3 on PB10,11: BMS
 * - PB6, PB7, PB8: hall sensors
 * - PA12: park button
 * - PB15: power button
 * - PC4,5: TM1638
 * - PB12: Headlight
 * - PB14: poweron
 * - PC9: left DRL
 * - PA8: right DRL
 * - PH0,1: 8 MHz oscillator
 *
 * ADC pins:
 * - PC0: I_battery
 * - PC1: T_driver
 * - PA0: V_battery
 * - PA4: I_phaseA
 * - PA5: I_phaseC
 */

// TODO: Figure out how to do "input methods". Some wheels will have controls like Begode,
// where there is just a power button and a park button, some wheels will have something
// like LeaperKim, with power + headlight + OK + next, etc etc. I have to figure out
// how to make it all coexist

pub const STARTUP_DELAY_MS: u64 = 1500;
// FIXME: NO BALANCE CONFIG, do at least a basic tune
pub const BALANCE_CONF: BalanceConfig = BalanceConfig {
    kp: 0,
    kp_expo: 0.0,
    ki: 0,
    kd_forward: 0,
    kd_backward: 0,
    dt: 0,
    rideassist: RideAssistConfig {
        accel_power_threshold: 0.0,
        accel_state_threshold: 0.0,
        braking_state_threshold: 0.0,
        state_hysteresis: 0,
    },
    integral_min: 0.0,
    integral_max: 0.0,
    out_min: 0,
    out_max: 0,
};

static mut ADC1_DMA_BUF: [u16; 6] = [0; 6];
static mut ADC2_DMA_BUF: [u16; 6] = [0; 6];
static mut ADC3_DMA_BUF: [u16; 6] = [0; 6];

pub struct BspPeripherals<'a> {
    poweron: gpio::Output<'a>,
    power_button: gpio::Input<'a>,
    park_button: gpio::Input<'a>,
    adc1: RingBufferedAdc<'a, ADC1>,
    adc2: RingBufferedAdc<'a, ADC2>,
    adc3: RingBufferedAdc<'a, ADC3>,
}

static mut BSP_PERIPH: MaybeUninit<BspPeripherals<'static>> = MaybeUninit::uninit();

#[allow(static_mut_refs)]
fn bsp_periph() -> &'static mut BspPeripherals<'static> {
    unsafe { &mut (*BSP_PERIPH.as_mut_ptr()) }
}

// Gather all peripherals required for opereration and initialize anything that
// needs to be initialized at this point. This function has to be called ONCE on boot.
#[allow(static_mut_refs)]
pub fn init<'a>(p: Peripherals) {
    let mut i_battery = p.PC0.degrade_adc();
    let mut t_driver = p.PC1.degrade_adc();
    let mut v_battery = p.PA0.degrade_adc();
    let mut i_phase_a = p.PA4.degrade_adc();
    let mut i_phase_c = p.PA5.degrade_adc();

    let mut adc1 = Adc::new(p.ADC1);
    let mut adc2 = Adc::new(p.ADC2);
    let mut adc3 = Adc::new(p.ADC3);

    let vrefint = adc2.enable_vrefint();
    let core_temp = adc3.enable_temperature();

    // TODO: Revisit the cycles part, maybe make it work better
    let mut adc1_rb = unsafe {
        adc1.into_ring_buffered(
            p.DMA2_CH0,
            &mut ADC1_DMA_BUF,
            [
                (i_phase_a, SampleTime::CYCLES112),
                (vrefint.degrade_adc(), SampleTime::CYCLES112),
                (core_temp.degrade_adc(), SampleTime::CYCLES112),
            ]
            .into_iter(),
            RegularConversionMode::Continuous,
        )
    };
    let mut adc2_rb = unsafe {
        adc2.into_ring_buffered(
            p.DMA2_CH2,
            &mut ADC2_DMA_BUF,
            [
                (i_phase_c, SampleTime::CYCLES112),
                (t_driver, SampleTime::CYCLES112),
            ]
            .into_iter(),
            RegularConversionMode::Continuous,
        )
    };
    let mut adc3_rb = unsafe {
        adc3.into_ring_buffered(
            p.DMA2_CH1,
            &mut ADC3_DMA_BUF,
            [
                (i_battery, SampleTime::CYCLES112),
                (v_battery, SampleTime::CYCLES112),
            ]
            .into_iter(),
            RegularConversionMode::Continuous,
        )
    };

    unsafe {
        BSP_PERIPH.write(BspPeripherals {
            poweron: gpio::Output::new(p.PB14, gpio::Level::Low, gpio::Speed::Medium),
            power_button: gpio::Input::new(p.PB15, gpio::Pull::Down),
            park_button: gpio::Input::new(p.PA12, gpio::Pull::Down),
            adc1: adc1_rb,
            adc2: adc2_rb,
            adc3: adc3_rb,
        });
    }
}

/*
 * MESC configuration
 */

// Peripheral table
pub const MESC_MOTOR_TIM: TimAdv = embassy_stm32::pac::TIM8;

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

// FIXME: TIM8 is not configured
#[interrupt]
fn TIM8_UP_TIM13() {
    unsafe {
        // FIXME: replace get_motor()
        // MESC_PWM_IRQ_handler(mesc::get_motor());
        // Clear update flag
        MESC_MOTOR_TIM.sr().modify(|w| w.set_uif(false));
    }
}

// FIXME: TIM3 is not configured
/// The balance loop interrupt
#[interrupt]
fn TIM3() {
    roles::control::balance_loop();
}

// FIXME: ADCs are not configured nor used
#[interrupt]
fn ADC() {
    unsafe {
        // FIXME: replace get_motor()
        // MESC_ADC_IRQ_handler(mesc::get_motor());
        // FIXME: ADC flags are NOT BEING RESET, IT WILL NOT FIRE AGAIN
    }
}

/*
 * Platform functions
 */

pub fn startup_successful() {
    bsp_periph().poweron.set_high();
}

#[allow(static_mut_refs)]
pub fn refresh_adc() {
    // unsafe {
    //     (&mut *MOTOR_TIM_DRIVER.as_mut_ptr()).set_high();
    // }
    unimplemented!()
}

pub fn refresh_adc_for_vphase() {
    unimplemented!()
}

pub async fn adc_read() {}
