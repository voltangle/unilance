# Arvenora UniLANCE

This is an electric unicycle firmware, built in Rust and actively uses code from the MESC
project for all FOC and motor control stuff. All credit for that goes to the creator,
David Molony.

## Status

The project is in a very early alpha (even before 0.1.0). The balancing algorithm works,
a lot of internal plumbing was done, but motor control is still not working as it should
(well, it *can* spin the motor, just roughly and with intermittent brownouts/overcurrents).
I'm working on making it work, as all the issues are basically ADC/Timer misconfigurations
and bad setups.

## Technologies used

- Rust (of course)
- embassy-rs as the async runtime and HAL
- littlefs as the filesystem
- postcard as the wire message and (sometimes) storage format
- MESC as the FOC implementation
- Madgwick algorithm as AHRS
- Compose Multiplatform for the LANCEmate app

## Building

The UniLANCE project uses a custom "build system" (effectively just a glorified task runner)
called "bear". To use it, you have to source the build environment script: `source build/env.sh`.
Note that Windows *not* through WSL is not supported, please use WSL when building this
thing on Windows.

To check out all available targets, run `bear target list`. To build for a target, run
`bear build <target-name>`, for example `bear build naegi`. For the entire commands list,
just run `bear` or `bear -h`, and it will show all functionality it has.

The naming convention for target names is just Danganronpa character surnames. That's it.

## Resource sharing between the Rust and C sections

MESC used here is a heavily modified version, which includes removal of ALL platform-dependent
code, and replacement with equivalent MESChal functions. These functions are implemented
on the Rust side with Embassy or direct register access, depending on performance requirements.
Before that, the project had to include CMSIS and make a thin implementation of ST HAL
that only implemented functionality that MESC was using. Fortunately, most of it
devolved to `_motor->mtimer.Instance->REG`, so I was able to easily refactor it all out
to MESChal variants.

To summarize: MESC doesn't own any peripherals by itself, any usage of them is done in Rust
code. Usage of those peripherals is done through MESChal functions, that are implemented
by Rust consumer code.

## Ports and boards

Each port is a separate binary in the root Cargo workspace, all named port_<mcu>. <mcu>
could be either a family of chips (stm32), or a specific one (stm32f405), depending
on the port itself. Each port has:

- Its own bin crate
- `bsp` module inside `src`
- MESC hook implementations that point to according BSP functions.

Each port has its own BSPs or Board Support Packages, which are basically different
configurations of the same port for different target hardware. Each BSP has:

- `[package.metadata.bear]` entry for target metadata
- Feature flags in its port that looks like `board_<bsp_name>`
- Its own module in `src/bsp` of its port, named `<bsp_name>.rs` if a single file or just
`<bsp_name>` if it's a folder.

## System architecture

The project is split into two main role - the control role and supervisor role.
Control is responsible for all mission-critical tasks, like motor control, balancing,
power alarms, etc etc, while supervisor, uhh, "supervises" and orchestrates the rest of
the system.

### Responsibilites of the control role
- Motor control
- Balancing
- Power alarms
- Calculation and processing of the torque map
- Communication with the supervisor via CAN (or in-memory channels, if both roles are combined)

### Responsibilities of the supervisor role
- Startup and shutdown
- If roles are split, handling power delivery to the control MCU
- Communication with control
- External buttons
- Displays
- BMS communications
- Relaying BMS system limits to control
- BLE/Wi-Fi
- USB PD

## Communication between nodes

The system is designed around the fact that the supervisor is the "master" uC, while every
other component in the system only work as "slaves". By "slaves" I mean the fact that they
don't do anything unless instructed to (with exceptions).

When the user presses the power button, the first uC to start up is always the supervisor,
and then all other chips are initialized, like control and BMS. The BMS uC is not usually
completely off tho, most of the time it's just in a power saving state.

## Tasks

### For v0.1 release

- [x] Figure out ports for different MCUs and boards
- [x] Implement support for both single and dual MCU boards
- [x] Make MESC work (no linker errors, all functions (at least theoretically) properly start)
    - [x] Refactor the configuration system
- [x] Properly do the ET Max config
- [x] Implement input method gestures
- [ ] Make the power button work (only turn on and off for now)
- [x] PI2D (Progressive + Integral + Double (sided) Derivative) part of the balance algo
- [ ] Implement simple in-RAM littlefs driver + littlefs orchestration
- [ ] Implement basic CORElink communication
    - [x] Initial hello exchange
    - [ ] Config transmission
    - [ ] (maybe) error reporting
- [x] CPU usage - total percentage
- [x] Make MESC NOT use direct register access (or be dependent on hardware at all), and
make it use functions defined in Rust instead

### Next

- [ ] Implement some kind of bootloader with embassy_boot
- [ ] Add Slint and a minimal UI
- [ ] Implement support for different displays
- [ ] Implement protocols with postcard
    - [ ] LANCElink (BLE) (postcard)
    - [ ] CORElink (internal CAN bus) (postcard)
- [ ] Allow to run core-supervisor in a "simulator", to test UI interactions
- [ ] Auto shutoff timer on idle
- [ ] Add current limiting via Ibus to MESC
- [ ] Motor temperature estimation using calculated resistance
- [ ] Testing
- [ ] Balance algorithm
    - [x] PI2D (Progressive + Integral + Double (sided) Derivative)
    - [ ] PI2D endstops
    - [ ] Tiltback algorithm
    - [ ] Angle cut out (both on pitch and roll axis)
    - [ ] Ride Assist
- [ ] Integrate Miri for UB checks
- [ ] Replace MESC with a custom FOC implementation written in Rust and do away with C in
the codebase (maybe)
- [ ] Remove temperature sensor handling from MESC, make Rust handle it instead
