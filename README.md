# UniLANCE

This firmware is built in Rust and actively uses code from the MESC project for all FOC
and motor control stuff. All credit for that goes to the creator, David Molony.

## Technologies used
- Rust (of course)
- embassy-rs as the async runtime and HAL
- littlefs as the filesystem
- postcard as the wire message format
- MESC as the FOC implementation

## Building

To check out all available targets, run `task all_targets`. To build for a target, run
`task <target-name>:build`, for example `task begode_etmax:build`.

The naming convention for target names is `<manufacturer>_<model>`. If there is no
name for a manufacturer, just skip it.

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

- Taskfile entry for building that target
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

## Init process

When the user presses the power button, they effectively enable power to the supervisor node
in the system. Supervisor executes its own init process, then enables power and sends a 
hello message to the control node. Control wakes up, asks for a config, does its init 
process, and either sends a success or an error back to the supervisor. Control never stores
any settings locally; everything is sent to it by the supervisor node.

## Tasks

### For v0.1 release

- [x] Figure out ports for different MCUs and boards
- [x] Implement support for both single and dual MCU boards
- [x] Make MESC work (no linker errors, all functions (at least theoretically) properly start)
    - [ ] Refactor the configuration system
- [x] Properly do the ET Max config
- [x] Implement input method gestures
- [ ] Make the power button work (only turn on and off for now)
- [x] PI2D (Progressive + Integral + Double (sided) Derivative) part of the balance algo
- [ ] Implement simple FLASH storage littlefs driver + littlefs orchestration
- [ ] Implement basic CORElink communication
    - [ ] Initial hello exchange
    - [ ] Config transmission
    - [ ] (maybe) error reporting
- [x] CPU usage - total percentage
- [ ] Buzzer support; adjust tone and volume, make the patterns platform independent
- [x] Make MESC NOT use direct register access (or be dependent on hardware at all), and
make it use functions defined in Rust instead

### Next

- [ ] Implement some kind of bootloader with embassy_boot
- [ ] Add Slint and a minimal UI
- [ ] Implement support for different displays
- [ ] Implement storage with littlefs
    - [ ] Do block device with embedded parity info
    - [ ] Do system config with a postcard type
- [ ] Implement protocols with postcard
    - [ ] LANCElink (BLE) (postcard)
    - [ ] CORElink (internal CAN bus) (postcard)
- [ ] Current sensor sanity checks and identifying if one or more are either damaged or dying
- [ ] Allow to run core-supervisor in a "simulator", to test UI interactions
- [ ] Auto shutoff timer on idle
- [ ] Add current limiting via Ibus to MESC
- [ ] Testing
- [ ] Balance algorithm
    - [x] PI2D (Progressive + Integral + Double (sided) Derivative)
    - [ ] PI2D endstops
    - [ ] Tiltback algorithm
    - [ ] Angle cut out (both on pitch and roll axis)
    - [ ] Ride Assist
- [ ] Integrate Miri for UB checks
- [ ] Support for different Smart BMSes
    - [ ] Begode charge board UART
    - [ ] Begode per-pack I2C
