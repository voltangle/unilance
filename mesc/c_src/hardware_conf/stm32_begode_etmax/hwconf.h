#pragma once

// FIXME: this was copypasted from some config to make the compiler stfu

#include <math.h>

#include "motors/begode_tile36-3_2.h"

#define PWM_FREQUENCY \
    10000  // This is half the VESC zero vector frequency; i.e. 20k is equivalent to VESC
           // 40k
// #define CUSTOM_DEADTIME 600 //ns

#define SHUNT_POLARITY -1.0

#define ABS_MAX_PHASE_CURRENT 100.0f
#define ABS_MAX_BUS_VOLTAGE 40.0f
#define ABS_MIN_BUS_VOLTAGE 24.0f
#define R_SHUNT 0.0005f
// ToDo need to define using a discrete opamp with resistors to set gain vs using one with
// a specified gain
#define R_SHUNT_PULLUP 4700.0f            // For discrete opamps
#define R_SHUNT_SERIES_RESISTANCE 150.0f  // For discrete opamps
#define R_VBUS_BOTTOM 1500.0f             // Phase and Vbus voltage sensors
#define R_VBUS_TOP 82000.0f
#define OPGAIN 16.0f
#define USE_INTERNAL_OPAMPS
#define ADC_OFFSET_DEFAULT 1870.0f;

#define MAX_ID_REQUEST 10.0f
#define MAX_IQ_REQUEST 30.0f

#define SEVEN_SECTOR  // Normal SVPWM implemented as midpoint clamp. If not defined, you
                      // will get 5 sector, bottom clamp
// #define DEADTIME_COMP
#define DEADTIME_COMP_V \
    5  // Arbitrary value for now, needs parametising.
       // Basically this is half the time between MOSoff and MOSon
       // and needs determining experimentally, either with openloop
       // sin wave drawing or by finding the zero current switching "power knee point"

// Inputs
#define GET_THROTTLE_INPUT \
    _motor->Raw.ADC_in_ext1 = hadc2.Instance->JDR4;  // Throttle for MP2 with F405 pill

// Use the Ebike Profile tool
// #define USE_PROFILE

// #define USE_FIELD_WEAKENING
#define USE_FIELD_WEAKENINGV2
#define FIELD_WEAKENING_CURRENT 20.0f
#define FIELD_WEAKENING_THRESHOLD 0.8f

/////////////////////Related to ANGLE ESTIMATION////////////////////////////////////////
#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_SENSORLESS
// #define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_HALL
// #define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_OPENLOOP
// #define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_ENCODER
// #define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_HFI

// #define INTERPOLATE_V7_ANGLE

#define USE_HFI
#define HFI_VOLTAGE 4.0f
#define HFI_TEST_CURRENT 0.0f
#define HFI_THRESHOLD 2.5f
#define DEFAULT_HFI_TYPE HFI_TYPE_NONE
// #define DEFAULT_HFI_TYPE HFI_TYPE_45
// #define DEFAULT_HFI_TYPE HFI_TYPE_D
// #define DEFAULT_HFI_TYPE HFI_TYPE_SPECIAL
#define MAX_MODULATION 0.5f

#define USE_HALL_START
#define HALL_VOLTAGE_THRESHOLD 1.5f
#define MIN_HALL_FLUX_VOLTS 5.0f

// #define USE_SPI_ENCODER //Only supports TLE5012B in SSC mode using onewire SPI on SPI3
// F405...
#define POLE_PAIRS 10
#define ENCODER_E_OFFSET 25000
#define POLE_ANGLE (65536 / POLE_PAIRS)

// #define USE_SALIENT_OBSERVER //If not defined, it assumes that Ld and Lq are equal,
// which is fine usually.

// If there are voltage sensors on phases.
// #define HAS_PHASE_SENSORS

#ifndef FIELD_WEAKENING_CURRENT
#define FIELD_WEAKENING_CURRENT \
    10.0f  // This does not set whether FW is used, just the default current
#endif

#ifndef FIELD_WEAKENING_THRESHOLD
#define FIELD_WEAKENING_THRESHOLD 0.8f
#endif

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

/////////////////////Related to COMMs INTERFACE//////////////////////////////
#define MESC_UART_USB MESC_USB
#define HW_UART huart3

/////////////////////Prototype stuff that does not really work
/// nicely//////////////////////////////

// #define USE_DEADSHORT //This can be used in place of the phase sensors for startup from
// running.
#define DEADSHORT_CURRENT \
    30.0f  // When recovering from tracking phase without phase sensors, the
           // deadshort function will short the phases
           // until the current exceeds this value. At this point, it calculates the Vd Vq
           // and phase angle Don't set too high, after 9PWM periods, it will run the calc
           // and start the motor regardless. This seems to work best with a higher
           // current bandwidth (~10krads-1) and using the non-linear observer centering.
           // Broadly incompatible with the flux observer
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
