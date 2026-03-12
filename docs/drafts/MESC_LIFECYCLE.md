# MESC lifecycle in UniLANCE

This document is a ChatGPT draft. It is meant to capture the current behavior of the local
MESC fork, not a final specification.

It is based on:

- the local `mesc` crate and C sources in this repository
- the current STM32 integration in `port-stm32`
- spot-checks against the original `davidmolony/MESC_Firmware` repository where local behavior
  looked ambiguous

## Big picture

UniLANCE does not use stock MESC as-is.

The local fork keeps the original MESC state machine, observers, FOC logic, and measurement
code in C, but moves platform ownership into Rust. In practice that means:

- Rust owns timers, ADCs, DMA, interrupts, and board bring-up
- C still owns the motor-control state machine and control math
- Rust feeds raw ADC samples and high-level requests into `MESC_motor_typedef`
- C computes the control result and asks the Rust HAL shims to apply PWM

Useful local entry points:

- `mesc/src/lib.rs`
- `mesc/c_src/MESC_Common/Src/MESCfoc.c`
- `mesc/c_src/MESC_Common/Inc/MESCfoc.h`
- `mesc/c_src/MESC_Common/Inc/MESCmotor_state.h`

## What the lifecycle looks like

At a high level, the local lifecycle is:

1. Rust constructs a zeroed/default `MESC_motor_typedef`
2. Rust calls `MESCfoc_Init()`
3. C enters `INITIALISING` and waits for the fast IRQ to finish current-offset calibration
4. once offsets look sane, C clears `UNINITIALISED_KEY`, enables output, and enters `TRACKING`
5. the slow loop decides when to transition from `TRACKING` into `RUN` or another state
6. the fast loop then runs the actual control law for the active `MotorState`
7. faults force `ERROR`, where PWM is broken and only limited tracking/observer work remains
8. the slow loop decides whether to remain in `ERROR`, recover to `TRACKING`, or move elsewhere

That is the intended MESC shape both locally and upstream. The differences in this repo are in
what Rust sets up around that shape and which branches are actually exercised.

## The real call chain in this repo

### Rust side

- `core-control` owns the long-lived `State`, including `motor: MESC_motor_typedef`
- control-role startup calls `motor.init()`
- the PWM ISR path calls `motor.foc_update()`
- the auxiliary loop calls `motor.foc_aux_update()`

In code:

- `core-control/src/lib.rs`
- `port-stm32/src/roles/control.rs`
- `port-stm32/src/tsp/naegi.rs`
- `mesc/src/lib.rs`

### C side

- `MESCfoc_Init()` sets defaults and blocks until init is complete
- `MESCfoc_fastLoop()` is the fast state-machine and FOC execution path
- `MESC_PWM_IRQ_handler()` writes the computed PWM state
- `MESCfoc_slowLoop()` handles control policy, limits, and high-level transitions

## Initialization details

### What `MESCfoc_Init()` does

`MESCfoc_Init()` performs a large amount of default setup:

- sets current offsets to ADC midpoint defaults
- sets `MotorState = INITIALISING`
- chooses default control mode and sensor mode
- initializes Hall, HFI, observer, BLDC, speed-control, and thermal structures
- sets option flags such as field weakening, observer type, PWM type, and startup sensor
- initializes current-controller bandwidth and observer gains
- sets `key_bits = UNINITIALISED_KEY`
- spins in a loop until the fast loop finishes inverter initialization
- recalculates gains and voltage scaling after init completes

Important local difference:

- upstream MESC uses more compile-time defaults and board init helpers
- this fork has removed most platform-specific setup from C and expects Rust to have the IRQ
  and ADC path alive before `MESCfoc_Init()` can finish

### Why init blocks

`MESCfoc_Init()` does not finish immediately. It waits in a `while` loop until the fast IRQ
side has gathered enough current samples to compute sensor offsets.

That means the following must already be true before `motor.init()` is called:

- PWM timer is running
- ADC/DMA sampling path is alive
- the fast interrupt can call `MESCfoc_fastLoop()` repeatedly

In the current STM32 port that appears to be true, which is why init can complete at all.

## Inverter initialization and offset calibration

While `MotorState == INITIALISING`, the fast loop calls `initialiseInverter()`.

That function:

- accumulates raw phase-current samples
- averages them over many fast-loop cycles
- computes the current-sensor offsets
- checks that the offsets are in a plausible range
- clears `UNINITIALISED_KEY`
- moves the motor to `TRACKING`
- enables PWM output

If the offsets look wrong, it calls `handleError(ERROR_STARTUP)` and the motor lands in
`ERROR`.

One notable local deviation from upstream:

- upstream does this after about 1000 cycles
- the local fork uses 5000 cycles

That makes the startup calibration more conservative, but also makes init completion depend on
more interrupt activity before the system can proceed.

## Fast loop responsibilities

`MESCfoc_fastLoop()` is the high-rate control path. In this repo it is called from the motor
timer ISR.

The fast loop does, in order:

- samples Hall state
- converts raw ADC values into physical currents/voltages
- applies fault checks
- computes Clarke/Park current transforms
- dispatches behavior based on `MotorState`
- updates PLL/electrical speed bookkeeping

The actual PWM register write happens separately through `MESC_PWM_IRQ_handler()` after the fast
loop has prepared the modulation state.

This split matters:

- `MESCfoc_fastLoop()` is the control/state core
- `MESC_PWM_IRQ_handler()` is the PWM-output side

## Slow loop responsibilities

`MESCfoc_slowLoop()` is the policy and state-transition layer.

It is responsible for things like:

- housekeeping
- safe-start logic
- control-mode handling
- dynamic gain/voltage-limit recomputation
- deciding transitions between `TRACKING`, `RUN`, `ERROR`, `SLAMBRAKE`, and so on

Think of it as the control supervisor inside MESC itself.

The fast loop enforces the current state every PWM cycle; the slow loop decides what the state
should be next.

## Motor states and what they mean

The important declared states are in `MESCmotor_state.h`.

### `INITIALISING`

- current-sensor offset calibration
- PWM held in break state while waiting for init completion

### `TRACKING`

- PWM output is effectively disabled
- MESC still tries to maintain an electrical angle estimate
- this is the usual waiting room before entering active drive

With phase sensors, tracking can observe phase voltages directly.
Without them, tracking is more limited.

### `RUN`

- active FOC mode
- the actual angle source depends on `MotorSensorMode`

Possible `MotorSensorMode` branches are:

- sensorless
- Hall
- open-loop
- absolute encoder
- incremental encoder

### `OPEN_LOOP_STARTUP`

- simple open-loop angle stepping plus FOC
- mainly a startup helper path rather than a full operating mode

### `DETECTING`

- hall-detection and hall-table measurement path
- can fall back toward sensorless measurement if Hall sensors are absent

### `MEASURING`

- RL and related motor-parameter measurement path
- forces special control behavior and test excitation

### `GET_KV`

- open-loop spin/observer measurement path for flux linkage estimation

### `TEST`

- deadtime, double-pulse, and other test helpers

### `RECOVERING`

- intended as a relock/restart path after loss of phase-sensor support or motion recovery
- upstream expects deadshort-based restart behavior here
- this fork currently short-circuits that path when deadshort is disabled

### `ERROR`

- software break is asserted
- PWM is disabled
- MESC may still keep limited angle-estimation work alive
- slow loop decides whether recovery is allowed

### `SLAMBRAKE`

- low-side braking / aggressive hold logic
- intended for strong braking or stationary hold behavior

### `IDLE`

- nothing active, PWM off

### `RUN_BLDC`

- BLDC commutation path instead of FOC
- present in C, but not well represented by the Rust wrapper types

## Control modes

MESC also has a separate `ControlMode` concept.

The important ones are:

- `TORQUE`
- `SPEED`
- `DUTY`
- `POSITION`
- `MEASURING`
- `HANDBRAKE`

These are handled mainly in the slow loop.

In the local fork:

- default control mode is torque
- Rust currently exposes q/d current requests, but not a full control-mode switch API
- in practice the live path is dominated by torque-mode behavior

## Sensor modes

The angle source used in `RUN` depends on `MotorSensorMode`.

### `SENSORLESS`

- uses the flux observer
- optionally uses Hall-assisted startup preload if enabled

### `HALL`

- uses the Hall state machine and Hall angle observer

### `OPENLOOP`

- uses `OLGenerateAngle()` to advance `FOCAngle` by `openloop_step`
- this is the simplest possible angle source: no sensor feedback, just a rotating field

### `ABSOLUTE_ENCODER` and `INCREMENTAL_ENCODER`

- use encoder angle as the electrical angle source

## What the local fork actually does today

The current Rust integration overrides some of the most important MESC defaults after C init:

- it forces `MotorSensorMode = OPENLOOP`
- it sets `FOC.openloop_step = 20`
- it hardcodes one motor profile in Rust

Also, the current control loop in the STM32 role is not yet feeding the balancing output into
MESC. It currently requests a constant positive q current in the auxiliary loop.

So the current effective path is much closer to:

- complete offset calibration
- sit in `TRACKING`
- receive a constant q-current request from Rust
- transition through `RECOVERING`
- enter `RUN`
- execute open-loop angle generation

That is useful for bring-up, but it is not representative of the more mature Hall or
sensorless closed-loop paths that upstream MESC is best known for.

## Upstream behavior that still matters

Spot-checking upstream `davidmolony/MESC_Firmware` confirms that the broad lifecycle model is
still the same:

- init blocks until IRQ-driven current-offset calibration completes
- `TRACKING` is the pre-run observer state
- `RUN` dispatches by sensor mode
- `RECOVERING` is meant as a real recovery path, not just a pass-through
- startup logic is strongly shaped by compile-time configuration in upstream

The local fork differs mainly by:

- removing board ownership from C
- moving configuration toward runtime Rust control
- hard-overriding sensor mode and motor profile on the Rust side
- simplifying some safety/startup behavior while bring-up is still in progress

## Things to pay attention to

### 1. The fork is no longer "pure MESC"

You cannot reason about lifecycle from upstream alone.

The actual lifecycle is now the combination of:

- C MESC state machine
- Rust-side initialization order
- Rust-side post-init overrides
- STM32 ISR and ADC plumbing

### 2. Initialization depends on interrupts already working

This is probably the single most important integration rule.

If the fast interrupt path is not alive before `MESCfoc_Init()` is called, init will never
finish.

### 3. Rust currently overrides the sensor strategy aggressively

Upstream defaults lean toward configured Hall/sensorless behavior.

This repo currently forces open-loop after init. That means many upstream assumptions about
startup and angle estimation are not what the running system actually uses.

### 4. Limit and input fields matter more than they look

MESC uses several request-limit and board-limit fields in slow-loop policy and fault logic.
If those are left zeroed or inconsistent, behavior becomes misleading very quickly.

In particular, pay attention to:

- `input_vars.max_request_Idq`
- `input_vars.min_request_Idq`
- voltage/current limits derived in `calculateVoltageGain()`
- `g_hw_setup` scaling and fault thresholds

### 5. `key_bits` are part of the effective lifecycle

The motor is not really "ready" just because `MotorState` changed.

`key_bits` gate whether MESC considers output requests valid. Any lifecycle analysis that looks
only at `MotorState` is incomplete.

### 6. ADC plumbing quality is safety-critical

The motor-control math assumes ADC channels map correctly to currents and bus voltage.
If the DMA channel layout is wrong, everything else can look conceptually correct while the
controller still behaves nonsensically or unsafely.

### 7. `RECOVERING` is currently fork-specific

Upstream expects a more meaningful motion recovery/deadshort path.
This fork currently relaxes that into a much simpler transition when deadshort is disabled.
That changes the real lifecycle materially.

### 8. `RUN_BLDC` is not well surfaced to Rust

If you ever exercise BLDC paths, the Rust type wrappers will not describe state cleanly.
That is easy to forget while reading only the Rust side.

### 9. Open-loop works only if `openloop_step` is nonzero

This sounds obvious, but it is central to bring-up in this fork.

If Rust forces `OPENLOOP` but forgets to set a useful `openloop_step`, MESC can make it all the
way to `RUN` while still failing to rotate the field.

### 10. Upstream startup knobs still exist, even if your fork bypasses them

`SLStartupSensor`, Hall-start preload, HFI, deadshort recovery, and measuring paths are all
still in the code. Even if you are not using them today, they still shape assumptions,
default branches, and fallback behavior.

## Practical mental model

The safest mental model for this fork is:

- Rust brings hardware to life
- C MESC waits for valid raw data and calibrates itself
- fast loop is the executor
- slow loop is the policy layer
- Rust can override major parts of the lifecycle after C init
- the real behavior is whatever the Rust integration currently forces, not whatever upstream
  MESC would ideally do on its own

## Recommended next checks

If you are bringing the system up or changing the integration, the most useful things to log or
inspect are:

- `MotorState`
- `ControlMode`
- `MotorSensorMode`
- `key_bits`
- `FOC.Idq_prereq`
- `FOC.Idq_req`
- `FOC.openloop_step`
- raw ADC channel mapping vs expected physical signals
- `g_hw_setup` gains and computed current/voltage limits

If those are wrong, the rest of MESC analysis usually does not matter yet.

## Short version

MESC in UniLANCE still follows the classic MESC lifecycle:

- initialize
- calibrate offsets
- track
- run
- fault/recover as needed

But the local fork changes the practical path in major ways through Rust-side hardware control,
runtime configuration, and post-init overrides. The biggest thing to remember is that you are
not integrating "upstream MESC" anymore; you are integrating a hybrid Rust/C controller where
the lifecycle is shared between both halves.
