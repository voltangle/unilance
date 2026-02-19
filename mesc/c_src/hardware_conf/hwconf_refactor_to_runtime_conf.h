/////////////////////Related to OBSERVER//////////////////////////////
#define USE_FLUX_LINKAGE_OBSERVER       // This tracks the flux linkage in real time,
#define USE_CLAMPED_OBSERVER_CENTERING  // Pick one of the two centering methods...
                                        // preferably this one

/////////////////////Prototype stuff that does not really work
/// nicely//////////////////////////////
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
