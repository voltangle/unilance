#pragma once

/*
 * This file is a template for a new hardware config (hwconf). All options are commented
 * out by default, so you need to choose all your options manually (with the exception
 * of some of them)
 */

// Choose your motor defaults here, or make a new configuration similar to any existing
// motor config.
#include "motors/begode_tile36-3_2.h"

// If there are voltage sensors on phases.
// #define HAS_PHASE_SENSORS

// Frequency at which the motor timer PWM will run. This is half the VESC zero vector
// frequency; i.e. 20k is equivalent to VESC 40k
// #define PWM_FREQUENCY 15000

// Motor timer dead time, in nanoseconds.
// #define CUSTOM_DEADTIME 0

// Inverter phase current threshold that triggers an error.
// #define ABS_MAX_PHASE_CURRENT 0.0f

// Upper inverter input voltage threshold that triggers an error.
// #define ABS_MAX_BUS_VOLTAGE 0.0f

// Lower inverter input voltage threshold that trigger an error.
// #define ABS_MIN_BUS_VOLTAGE 0.0f

// ToDo need to define using a discrete opamp with resistors to set gain vs using one with
// a specified gain
//  TODO: figure out if it's needed or not
#define OPGAIN 16.0f

// Offset for ADC values, so negative current values can also be read.
// #define ADC_OFFSET_DEFAULT 1870.0f;

// Max current that can be requested in FOC.
// #define MAX_IQ_REQUEST 30.0f

// Min current that can be requested in FOC. Unless actually needed, just leave as is.
#define MIN_IQ_REQUEST (-MAX_IQ_REQUEST)

// Normal SVPWM implemented as midpoint clamp. If not defined, you will get 5 sector,
// bottom clamp #define SEVEN_SECTOR

// #define DEADTIME_COMP
#define DEADTIME_COMP_V \
    5  // Arbitrary value for now, needs parametising.
       // Basically this is half the time between MOSoff and MOSon
       // and needs determining experimentally, either with openloop
       // sin wave drawing or by finding the zero current switching "power knee point"

// Enable field weakening.
// WARNING: Old (and sometimes unsafe) version, do not enable unless you know what you
// are doing! Refer to MESC documentation for more details.
// #define USE_FIELD_WEAKENING

// Enable v2 field weakening. Refer to MESC documentation for more details.
// NOTE: It's the better version compared to v1 field weakening. Use this unless you
// absolutely know what you're doing!
// #define USE_FIELD_WEAKENINGV2

// Maximum available field weakening current.
// #define FIELD_WEAKENING_CURRENT 0.0f

// The "PWM" (in EUC terms) threshold where field weakening starts to have an effect.
// #define FIELD_WEAKENING_THRESHOLD 0.0f

// TODO: Add documentation
// #define INTERPOLATE_V7_ANGLE

// For more information about HFI, refer to MESC documentation. This comment applies to
// all options related to HFI.
// #define USE_HFI
// #define HFI_VOLTAGE 4.0f
// #define HFI_TEST_CURRENT 0.0f
// #define HFI_THRESHOLD 2.5f
// #define DEFAULT_HFI_TYPE HFI_TYPE_NONE
// //#define DEFAULT_HFI_TYPE HFI_TYPE_45
// //#define DEFAULT_HFI_TYPE HFI_TYPE_D
// //#define DEFAULT_HFI_TYPE HFI_TYPE_SPECIAL
// #define MAX_MODULATION 0.5f

#define HALL_VOLTAGE_THRESHOLD 1.5f
#define MIN_HALL_FLUX_VOLTS 5.0f

// If not enabled, it assumes that Ld and Lq are equal, which is fine usually.
// #define USE_SALIENT_OBSERVER

/////////////////////Related to CIRCLE LIMITATION////////////////////////////////////////
// #define USE_SQRT_CIRCLE_LIM //Use for high PWM frequency (less clock cycles) or try if
// stability issues seen with Vd favouring option (unlikely)
#define USE_SQRT_CIRCLE_LIM_VD  // Use for Field weakening

// #define USE_MTPA

/////////////////////Related to ONLINE PARAMETER ESTIMATION//////////////////////////////
#ifndef LR_OBS_CURRENT
#define LR_OBS_CURRENT \
    0.1 * MAX_IQ_REQUEST  // Inject this much current into the d-axis at the slowloop
                          // frequency and observe the change in Vd and Vq Needs to be a
                          // small current that does not have much effect on the running
                          // parameters.
#endif

/////////////////////Related to OBSERVER//////////////////////////////
#define USE_FLUX_LINKAGE_OBSERVER  // This tracks the flux linkage in real time,
#define MAX_FLUX_LINKAGE DEFAULT_FLUX_LINKAGE * 2.0f  // Sets the limits for tracking.
#define MIN_FLUX_LINKAGE \
    DEFAULT_FLUX_LINKAGE * 0.7f  // Faster convergence with closer start points
#define FLUX_LINKAGE_GAIN \
    10.0f *               \
        sqrtf(            \
            DEFAULT_FLUX_LINKAGE)  //*(DEFAULT_FLUX_LINKAGE*DEFAULT_FLUX_LINKAGE)*PWM_FREQUENCY

// #define USE_NONLINEAR_OBSERVER_CENTERING //This is not a preferred option, since it
// relies on gain tuning and instability, which is precisely what the original observer
// intended to avoid. Also, incompatible with flux linkage observer for now...
#define NON_LINEAR_CENTERING_GAIN 5000.0f
#define USE_CLAMPED_OBSERVER_CENTERING  // Pick one of the two centering methods...
                                        // preferably this one

/////////////////////Prototype stuff that does not really work
/// nicely//////////////////////////////

// #define USE_DEADSHORT //This can be used in place of the phase sensors for startup from
// running. #define DEADSHORT_CURRENT 30.0f	//When recovering from tracking phase
// without phase sensors, the
// deadshort function will short the phases
// until the current exceeds this value. At this point, it calculates the Vd Vq and phase
// angle Don't set too high, after 9PWM periods, it will run the calc and start the motor
// regardless. This seems to work best with a higher current bandwidth (~10krads-1) and
// using the non-linear observer centering. Broadly incompatible with the flux observer
// Only works for forward direction presently
//^^WIP, not completely stable yet

/* Temperature parameters */
#define MESC_TEMP_MOS_R_F 10000.0f
#define MESC_TEMP_MOS_METHOD TEMP_METHOD_STEINHART_HART_BETA_R
#define MESC_TEMP_MOS_SCHEMA TEMP_SCHEMA_R_F_ON_R_T
#define MESC_TEMP_MOS_SH_BETA 3437.864258f
#define MESC_TEMP_MOS_SH_R 0.098243f
#define MESC_TEMP_MOS_SH_R0 10000.0f

#define MESC_TEMP_MOTOR_R_F 10000.0f
#define MESC_TEMP_MOTOR_METHOD TEMP_METHOD_STEINHART_HART_BETA_R
#define MESC_TEMP_MOTOR_SCHEMA TEMP_SCHEMA_R_F_ON_R_T
#define MESC_TEMP_MOTOR_SH_BETA 3437.864258f
#define MESC_TEMP_MOTOR_SH_R 0.098243f
#define MESC_TEMP_MOTOR_SH_R0 10000.0f
