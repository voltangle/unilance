# Naming convention and architecture

The core organisation of the project is done with ports, targets, and target variants. Ports are
crates that implement all foundational stuff for running on specific families of hardware,
like `port-stm32`. Targets themselves are kind of the same level as ports, in a sense
that they can exist in multiple ports at the same time, for example when a target is built
with a split role design, and one uC is an STM32, while another is an nRF52 for example,
and define the hardware architecture and design that UniLANCE is supposed to run on.
Target variants are, well, a variation of the target, but ONLY in terms of tuning and setup.
Variants only change how the software is configured at runtime, but not the firmware itself.
Basically, it's variants of the system partition in the filesystem.

So, TL;DR: ports implement foundations for different uC families, targets implement specific
hardware (boards) support, target variants give different tunes and configurations on top
of those targets.

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
| Phase voltage sensing | abscent |
| Input current sensing | present |
| Motor driver temperature sensor | present (NTC) |
| Motor temperature sensor | abscent |
| FETs | x48 HYG100N20 in DPAK packaging |
| Current sensors | CC6920BSO-50A + dual 0.3 Ohm parallel shunts |
| FET drivers | EG2186 |

