#pragma once

/*
 * This file is a template for a new hardware config (hwconf). All options are commented
 * out by default, so you need to choose all your options manually (with the exception
 * of some of them)
 */

// Choose your motor defaults here, or make a new configuration similar to any existing
// motor config.
#include "motors/begode_tile36-3_2.h"

// Motor timer dead time, in nanoseconds.
// #define CUSTOM_DEADTIME 0

// Inverter phase current threshold that triggers an error.
// #define ABS_MAX_PHASE_CURRENT 0.0f

// Upper inverter input voltage threshold that triggers an error.
// #define ABS_MAX_BUS_VOLTAGE 0.0f

// Lower inverter input voltage threshold that trigger an error.
// #define ABS_MIN_BUS_VOLTAGE 0.0f

// Max current that can be requested in FOC.
// #define MAX_IQ_REQUEST 30.0f

// Min current that can be requested in FOC. Unless actually needed, just leave as is.
#define MIN_IQ_REQUEST (-MAX_IQ_REQUEST)

// Maximum available field weakening current.
// #define FIELD_WEAKENING_CURRENT 0.0f

// The "PWM" (in EUC terms) threshold where field weakening starts to have an effect.
// #define FIELD_WEAKENING_THRESHOLD 0.0f

#define HALL_VOLTAGE_THRESHOLD 1.5f

/////////////////////Related to ONLINE PARAMETER ESTIMATION//////////////////////////////
#ifndef LR_OBS_CURRENT
#define LR_OBS_CURRENT \
    0.1 * MAX_IQ_REQUEST  // Inject this much current into the d-axis at the slowloop
                          // frequency and observe the change in Vd and Vq Needs to be a
                          // small current that does not have much effect on the running
                          // parameters.
#endif

/////////////////////Related to OBSERVER//////////////////////////////
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
