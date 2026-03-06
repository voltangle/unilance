# Naming convention and architecture

The core organization of the project is built around three things: ports, targets, and
target variants. That's the whole game here.

## Ports

Ports are crates that provide the foundational support for a specific MCU family. For
example, `port-stm32` supports the STM32 family, and a future `port-nrf` would support the
nRF family.

Ports should not contain runtime application logic. Their job is to do bootstrapping,
low-level initialization, and generally provide the environment in which TSPs (Target
Support Packages) are implemented.

## TSPs

TSPs are components inside a port that define how a specific hardware setup should be used.
A TSP is responsible for implementing the MESC HAL, configuring peripherals such as timers
and I/O, setting up drivers for components such as IMUs and displays, and exposing binding
functions that the rest of the system uses to interact with that hardware.

Each TSP is tied to exactly one target, although TSPs can absolutely reuse shared code when
multiple targets have common hardware support requirements.

## Targets

Targets represent the concrete hardware that UniLANCE runs on. A single target should map to
one specific hardware configuration, and it should not rely on compile-time feature toggles
for behavioral variation. If you have to start playing feature-flag games to make one
target cover multiple hardware setups, it is probably not one target anymore.

If supporting a piece of hardware would require compile-time feature flags to alter the
target's behavior, that hardware should be split into multiple targets instead. The point is
to keep targets explicit and avoid sneaky build-time variation.

Each target also defines its role configuration. A role describes how responsibilities are
split between MCUs on the target hardware. As of writing this doc, UniLANCE has two roles:

- `control`, responsible for mission-critical stuff such as motor control, balance, and safety handling
- `supervisor`, responsible for everything else, and like the name implies, it supervises the system and monitors its health

### Example: adding a target

Imagine we need to add support for the Begode Race board. The board uses a single MCU that
handles basically everything, from motor control to lights and displays. The MCU is an
STM32F405, so it belongs to the STM32 port. In that case, the flow looks something like
this:

- Choose a target name, for example `komaeda`
- Create a TSP file in `port-stm32/src/tsp/komaeda.rs`
- Add the file to `port-stm32/src/tsp/mod.rs`, and gate the `mod` statement with your
target feature flag
- Add a `[package.metadata.bear]` entry:

```toml
komaeda = { combined = "thumbv7em-none-eabihf" }
```

- And finally, add a new feature to `port-stm32`:

```toml
target_komaeda = [
    "role_supervisor",
    "role_control",
    "stm32f405rg",
    "drivers/mpu6500",
    ...
]
```

The Bear metadata entry declares that the target `komaeda` is implemented by this port and
that it exposes a `combined` role. Role names are not really governed by Bear itself. Bear
just reads the metadata of all `port-*` crates, looks for target declarations, and then uses
those role names when building firmware. Nothing too magical there.

### Bear feature naming

If only one role is defined for a target in a given port, Bear omits the role suffix when
constructing the target feature name. For example, if `komaeda` only had a `control` role,
Bear would just use `target_komaeda`.

If multiple roles exist for the same target, Bear includes the role suffix. For example, if
`komaeda` defines both `control` and `supervisor`, Bear will use
`target_komaeda_control` and `target_komaeda_supervisor`.

## Target Variants

UniLANCE uses a filesystem to store configuration, logs, state, and other persistent data.
That filesystem is split into two main partitions: system and data. The filesystem
architecture is described in [`FILESYSTEM.md`](./FILESYSTEM.md).

A target variant is only a different system partition configuration. A variant may change
UniLANCE configuration, but it must not change how the firmware is compiled or where it
runs. Bear intentionally does not emit Cargo feature flags for target variants, specifically
to enforce that rule. If a "variant" needs compile-time behavior changes, then it is not a
variant anymore.

As of writing this doc, the filesystem architecture is not fully defined yet, so the exact
mechanism by which a variant influences the system partition is still undecided. So for now,
the important part is the rule: variants change configuration, not compilation. That's the
part that matters.

## Naming convention

- Ports use `port-<family>`, for example `port-stm32` or `port-esp32c`
- Targets use Danganronpa character surnames. Yes, I'm not joking. Examples: `naegi`, `kirigiri`, `komaeda`, or `sayonji`
- Target variants depend on the target and use case, but they usually look like `naegi/panther`

So, TL;DR: ports provide support for MCU families, targets describe concrete hardware through
TSPs inside those ports, and target variants provide configuration differences on top of a
target without introducing build-time variation. Simple enough.

# List of available targets

## `naegi`

Compatible models:

- Begode ET Max
- Begode Panther
- Extreme Bull GT Pro
- Extreme Bull GT Pro+

| Key | Spec sheet |
| --- | ----- |
| Architecture | combined role |
| MCU | STM32F405RG |
| Phase current sensing | dual-sensor (phases A and C) |
| Phase voltage sensing | absent |
| Input current sensing | present |
| Motor driver temperature sensor | present (NTC) |
| Motor temperature sensor | absent |
| FETs | x48 HYG100N20 in DPAK packaging |
| Current sensors | CC6920BSO-50A + dual 0.3 Ohm parallel shunts |
| FET drivers | EG2186 |
