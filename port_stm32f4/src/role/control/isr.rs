use embassy_stm32::interrupt;

/*
 * Interrupts
 */

#[allow(non_snake_case)]
#[interrupt]
fn TIM8_UP_TIM13() {
    unimplemented!()
}

// TODO: Figure out ISR in general. Nothing is implemented here lol
