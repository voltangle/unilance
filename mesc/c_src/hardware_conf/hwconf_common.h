#pragma once

/*
 * This here is the shared config between all hardware configs. Do not change anything
 * here unless it's needed for absolutely ALL targets! Almost always you only need to
 * override these in your hardware config for your own target.
 */

#define NUM_MOTORS 1
#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_SENSORLESS
#define USE_HALL_START
