# UniLANCE

This firmware is built in Rust and actively uses code from the MESC project for all FOC
and motor control stuff. All credit for that goes to the creator, David Molony.

## Building

To check out all available targets, run `task all_targets`. To build for a target, run
`task build_<target-name>`, for example `task build_stm32f4_begode_etmax`.

The naming convention for target names is `<mcu>_<manufacturer>_<model>`. If there is no
name for a manufacturer, just skip it. Naming inside a port binary doesn't need the MCU
section.

## Resource sharing between the Rust and C sections

The Rust section is the one owning almost all the peripherals (through Embassy), it's
responsible for initialization, management, etc etc. Only exceptions are peripherals used
by MESC (C section), like the advanced timer (TIM1/8 or whatever is on that platform).
MESC by itself depends on ST HAL, but its usage of it is limited to literately
`hperiph->Instance->REG`. At this point it can just abandon the HAL and use just CMSIS,
but here we are. This allow me to make a thin HAL implementation that only has definitions
for some peripherals MESC uses, and then those are passed to MESC by UniLANCE initialization
code. Nice.

## Current tasks

- [x] Figure out ports for different MCUs and boards
- [ ] Make MESC work (no linker errors, all functions properly start)
