#define DEADTIME_COMP

// Normal SVPWM implemented as midpoint clamp. If not defined, you will get 5 sector,
// bottom clamp
#define SEVEN_SECTOR

#define INTERPOLATE_V7_ANGLE

// If not enabled, it assumes that Ld and Lq are equal, which is fine usually.
#define USE_SALIENT_OBSERVER

// #define USE_SQRT_CIRCLE_LIM //Use for high PWM frequency (less clock cycles) or try if
// stability issues seen with Vd favouring option (unlikely)
#define USE_SQRT_CIRCLE_LIM_VD  // Use for Field weakening

/////////////////////Related to CIRCLE LIMITATION////////////////////////////////////////
#define USE_SQRT_CIRCLE_LIM  // Use for high PWM frequency (less clock cycles) or try if
// stability issues seen with Vd favouring option (unlikely)
#define USE_SQRT_CIRCLE_LIM_VD  // Use for Field weakening

#define USE_MTPA

/////////////////////Related to OBSERVER//////////////////////////////
#define USE_FLUX_LINKAGE_OBSERVER       // This tracks the flux linkage in real time,
#define USE_CLAMPED_OBSERVER_CENTERING  // Pick one of the two centering methods...
                                        // preferably this one

/////////////////////Prototype stuff that does not really work
///nicely//////////////////////////////
#define USE_DEADSHORT  // This can be used in place of the phase sensors for startup from
// running.
#define DEADSHORT_CURRENT 30.0f  // When recovering from tracking phase
// without phase sensors, the
// deadshort function will short the phases
// until the current exceeds this value. At this point, it calculates the Vd Vq and phase
// angle Don't set too high, after 9PWM periods, it will run the calc and start the motor
// regardless. This seems to work best with a higher current bandwidth (~10krads-1) and
// using the non-linear observer centering. Broadly incompatible with the flux observer
// Only works for forward direction presently
//^^WIP, not completely stable yet
