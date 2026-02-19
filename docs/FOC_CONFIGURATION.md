This page should have all documentation for configuring the FOC backend (MESC) in UniLANCE.

# Static hardware configuration

Some configuration variables have default values that have to be defined. They are defined
as constants in the consuming code, and then automatically set when creating a
`mesc::Motor` instance. For documentation on those, please check out `mesc::Hardware`.

# Config variables in `MESC_motor_typedef`

## Use phase sensors (`motor->options.use_phase_sensors`)

> Formerly called `HAS_PHASE_SENSORS`.

If enabled, will continue tracking the motor even when commutation is off. For our
usecase it's not as useful, because we always supply power to the motor no matter what,
and if we need tracking when off (for example when parked), there are always hall sensors.

## Deadtime compensation (`motor->options.use_deadtime_compensation` and `motor->FOC.deadtime_comp`)

> Formerly called `DEADTIME_COMP` and `DEADTIME_COMP_V`.

Basically this is half the time between MOSoff and MOSon and needs determining
experimentally, either with openloop sin wave drawing or by finding the zero current
switching "power knee point" Not defining this uses 5 sector and overmodulation
compensation 5 sector is harder on the low side FETs (for now) but offers equal
performance at low speed, better at high speed.

Needs determining through `TEST_TYP_DEAD_TIME_IDENT`. (TODO: find out what it is)

## Interpolate V7 angle (`motor->options.interpolate_v7_angle`)

> Type: bool

TODO: document

## Non-linear centering gain (`motor->m.non_linear_centering_gain`)

> Type: f32

TODO: document

## 
