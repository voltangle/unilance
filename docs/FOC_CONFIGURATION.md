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

## Use salient observer (`motor->options.use_salient_observer`)

> Formerly `USE_SALIENT_OBSERVER`

If not enabled, it assumes that Ld and Lq are equal, which is fine usually.

## Non-linear centering gain (`motor->m.non_linear_centering_gain`)

> Type: f32

TODO: document

## Absolute max phase current (`motor->limits.abs_max_phase_current`)

Current which throws an overcurrent error (`ERROR_OVERCURRENT_PH(A|B|C)`).

## Absolute max bus voltage (`motor->limits.abs_max_bus_voltage`)

Input voltage that throws an overvoltage error (`ERROR_OVERVOLTAGE`).

## LR observer current (not yet made)

Inject this much current into the d-axis at the slowloop frequency and observe the change
in Vd and Vq Needs to be a small current that does not have much effect on the running
parameters.

Recommended default value: 0.1 * `MAX_IQ_REQUEST`.

## Square root circle limiter (`motor->options.sqrt_circle_lim`)

> Values: `SQRT_CIRCLE_LIM_OFF`, `SQRT_CIRCLE_LIM_ON`, `SQRT_CIRCLE_LIM_VD`

Use LIM_ON for high PWM frequency (less clock cycles) or try if stability issues seen with
Vd favouring option (unlikely). Use LIM_VD with field weakening.

## MTPA mode (`motor->options.mtpa_mode`)

> Values: `MESC_MTPA_NONE`, `MESC_MTPA_REQ`, `MESC_MTPA_MAG`, `MESC_MTPA_Q`

Maximum Torque Per Amp (MTPA). Check out `MESCfoc.c` for more details on how this works.
