#![allow(unused)]

pub static ADC_AWD: u32 = 1;
pub static ADC_JEOS: u32 = 1 << 2;
pub static ADC_JEOC: u32 = 1 << 3;
// TIM8_TRGO, aka ADC conversions are driven by the motor control timer.
// Refer to RM0090 Rev 21 (F405 reference manual) table 69 (nice) in
// section 13.6 for details.
#[cfg(feature = "stm32f405rg")]
pub static ADC_CONV_TRIG_TIM8_TRGO: u8 = 0b1110;
