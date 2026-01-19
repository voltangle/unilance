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
code. This is exactly what I did, for example in port_stm32f4/src/sthal.rs

## Ports and boards

Each port is a separate binary in the root Cargo workspace, all named port_<mcu>. <mcu>
could be either a family of chips (stm32f4), or a specific one (stm32f405), depending
on the port itself. Each port has:

- Its own bin crate
- `bsp` module inside `src`
- MESC hook implementations that point to according BSP functions.

Each port has its own BSPs or Board Support Packages, which are basically different
configurations of the same port for different target hardware. Each BSP has:

- Taskfile entry for building that target
- Feature flags in its port that looks like `board_<bsp_name>`
- Feature flags in `mesc` and `core` that look like `<port_name>_<bsp_name>`
- Its own module in `src/bsp` of its port, named `<bsp_name>.rs` if a single file or just
`<bsp_name>` if it's a folder.

## Tasks

- [x] Figure out ports for different MCUs and boards
- [x] Implement support for both single and dual MCU boards
- [ ] Make MESC work (no linker errors, all functions (at least theoretically) properly start)
- [ ] Balance algorithm (at least preliminary)
    - [x] PI2D (Progressive + Integral + Double (sided) Derivative)
    - [ ] PI2D endstops
    - [ ] Tiltback algorithm
    - [ ] Angle cut out
    - [ ] Ride Assist
- [ ] Implement some kind of bootloader with embassy_boot
- [ ] Properly do the ET Max config
- [ ] CPU usage - total percentage and per-task/ISR breakdown
- [ ] Make MESC NOT use direct register access (or be dependent on hardware at all), and
make it use functions defined in Rust instead
- [ ] Testing
