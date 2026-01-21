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

MESC used here is a heavily modified version, which includes removal of ALL platform-dependent
code, and replacement with equivalent MESChal functions. These functions are implemented
on the Rust side with Embassy or direct register access, depending on performance requirements.
Before that, the project had to include CMSIS and make a thin implementation of ST HAL
that only implemented functionality that MESC was using. Fortunately, most of it
devolved to `_motor->mtimer->Instance->REG`, so I was able to easily refactor it all out
to MESChal variants.

To summarize: MESC doesn't own any peripherals by itself, any usage of them is done in Rust
code. Usage of those peripherals is done through MESChal functions, that are implemented
by Rust consumer code.

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
    - [ ] Refactor the configuration system
- [ ] Balance algorithm (at least preliminary)
    - [x] PI2D (Progressive + Integral + Double (sided) Derivative)
    - [ ] PI2D endstops
    - [ ] Tiltback algorithm
    - [ ] Angle cut out
    - [ ] Ride Assist
- [ ] Implement some kind of bootloader with embassy_boot
- [ ] Properly do the ET Max config
- [ ] Add Slint and a minimal UI
- [ ] Add support for different displays and input method types
- [ ] Allow to run core_supervisor in a "simulator", to test UI interactions
- [ ] CPU usage - total percentage and per-task/ISR breakdown
- [ ] Make MESC NOT use direct register access (or be dependent on hardware at all), and
make it use functions defined in Rust instead
- [ ] Testing
