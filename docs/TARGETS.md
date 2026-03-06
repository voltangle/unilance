# Naming convention and architecture

The core organisation of the project is done with ports, targets, and target variants.

Ports are crates that implement the foundational stuff for a specific platform, like
`port-stm32` is a port for the STM32 family of MCUs, and a supposed future `port-nrf`
will support the nRF family. Ports do not have any "runtime" logic, their job is to do
proper bootstrapping, initialization, and giving an environment to implement TSPs
(Target Support Packages).

Speaking of - TSPs are components inside a port that define how specific hardware setups
should be used. A TSP is responsible for implementing the MESC HAL, configuring peripherals
like timers and I/O, setting up drivers for components like IMUs and displays, and
implementing binding functions that are then called by the outside system to actually
interact with those components.
TSPs are always tied to just one target, although one TSP can reuse code from another
if they have a "shared module" so to speak.

Speaking of targets - targets represent the hardware UniLANCE runs on. A single target
is only for a specific set of hardware, it should NOT have any compile-time features in it.
If a target needs some features that can be only tweaked using compile-time features,
the target needs to be split up, so that any compile-time variation abilities are removed.
Each target also defines its "role" configuration. Roles, in this case, mean how responsibilities
are split between MCUs on the target hardware. As of writing this doc, UniLANCE has two
roles, `control` and `supervisor`. `control`, for example, is responsible for all
mission-critical stuff, like motor control, balance, safety alarms, etc etc, while
`supervisor` does everything else, and like the name implies, `supervisor`, uhh,
supervises the system and monitors its health. I will go into detail on roles a bit later,
back to targets - it would be easier to explain with an example how it all works.

Imagine we need to add support for the Begode Race board. The board is built with a single
MCU that does basically everything, from motor control to lights and displays. The MCU itself
is an STM32F405, so its in the STM32 family. In this case, we do this:

- Choose a target name, for example `komaeda`
- Define a TSP (Target Support Package) file in `port-stm32/src/tsp/komaeda.rs`
- Add a `[package.metadata.bear]` entry:
```toml
komaeda = { combined = "thumbv7em-none-eabihf" }
```

- Add a new feature to `port-stm32`:
```toml
target_komaeda = [
    "role_supervisor",
    "role_control",
    "stm32f405rg",
    "drivers/mpu6500",
    ...
]
```

The Bear metadata entry defines that the target `komaeda` uses this port, and defines a
"combined" role. Roles names are not "governed" by Bear, it only checks the metadata of
all `port-*` crates, looks for target declarations, and uses the names in those declarations
when building the firmware itself. 

> NOTE: if there is only one role defined for a target in a given port, Bear will omit the
> role suffix when adding the target feature. For example, if you only have a `control`
> role defined for `komaeda`, Bear will use the `target_komaeda` feature. But, if you have
> both `control` and `supervisor` defined, it will use `target_komaeda_control` and
> `target_komaeda_supervisor` features instead.

Now that we've covered that, lets go to the last topic - target variants.

UniLANCE uses a filesystem for storing configs, logs, state, etc etc, and that filesystem
is split into two main partitions - system and data. The filesystem architecture is
explained in [here](FILESYSTEM.md), so going back to target variants - a variant
is ONLY a different system partition configuration. A target variant can *only* change
the *configuration* of UniLANCE, not how its compiled or where. Bear will not emit
any Cargo feature flags when building the firmware related to target variants, specifically
to enforce this rule. As of writing this doc, the filesystem architecture is not yet defined,
so any details on how *exactly* a target variant can influence the system partition is unknown
yet.

So, TL;DR: ports implement foundations for different uC families, targets implement specific
hardware support with TSPs (Target Support Packages) in those ports, and target variants
give different tunes and configurations on top of those targets.

As of the naming convention:

- For ports, its `port-<family>`, like if a port supports all
STM32s, it's gonna be `port-stm32`, and if it supports just the ESP32C series, it will be
`port-esp32c`. 
- For targets, it's Danganronpa character surnames. Yes, i'm not joking. Danganronpa
characters. Examples: `naegi`, `kirigiri`, `komaeda`, `sayonji`, etc.
- For target variants, it depends on the specific situation, but usually it's something
like `naegi/panther` for a Begode Panther variant of the `naegi` target.

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

