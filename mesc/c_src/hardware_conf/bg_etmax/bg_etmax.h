/*
 * Run18-24s
 *
 *  Created on: Aug 2024
 *      Author: HPEnvy
 */

#include "stm32f405xx.h"
#include <stdint.h>

#ifndef BOARD_BG_ETMAX_H
#define BOARD_BG_ETMAX_H
//Pick a motor for default
#define BSM_AXIAL//Q4120_700KV//Q3513_700KV//MCMASTER_70KV_8080//QS165//CA120//
#define PWM_FREQUENCY 20000
#define CUSTOM_DEADTIME 600 //ns, MAX 1500ns! implementation in MESCInit().

#define SHUNT_POLARITY -1.0f

#define ABS_MAX_PHASE_CURRENT 660.0f //We set this as the board abs max, and the firmware sets the value actually used depending on the input setpoints with this as a maximum.
#define ABS_MAX_BUS_VOLTAGE 110.0f
#define ABS_MIN_BUS_VOLTAGE 40.0f
#define R_SHUNT 1.0f
#define OPGAIN 1.65f/750.0f //750A dual split 6.5mm gap, hall sensors

#define R_VBUS_BOTTOM 6200.0f //Phase and Vbus voltage sensors
#define R_VBUS_TOP 300000.0f

#define MAX_ID_REQUEST 2.0f
#define MAX_IQ_REQUEST 10.0f
#define MIN_IQ_REQUEST -10.0f
#define DEFAULT_CONTROL_MODE MOTOR_CONTROL_MODE_TORQUE
#define ADC1OOR 4090


#define DEADTIME_COMP		//This injects extra PWM duty onto the timer which effectively removes the dead time.
#define DEADTIME_COMP_V 10
//#define MAX_MODULATION 1.10f //Use this with 5 sector modulation if you want extra speed
//Inputs
#define GET_THROTTLE_INPUT2 	_motor->Raw.ADC_in_ext2 = 0.99f*_motor->Raw.ADC_in_ext1 + 0.01f*hadc1.Instance->JDR3;  // Throttle2 for Run on Pa4
#define GET_THROTTLE_INPUT 		_motor->Raw.ADC_in_ext1 = 0.9f*_motor->Raw.ADC_in_ext2 + 0.1f*ADC1_buffer[0];  // Throttle2 for Run on PA3

#define GET_FETU_T 			_motor->Raw.MOSu_T = hadc2.Instance->JDR3; //Temperature on PC4, ADC14
#define GET_FETV_T 			_motor->Raw.MOSv_T = hadc2.Instance->JDR4; //Temperature on PC5, ADC15

#define GET_MOTOR_T _motor->Raw.Motor_T = ADC2_buffer[3]; //MotorT for Run on PB1, ADC2-9
//#define USE_FIELD_WEAKENING
#define USE_FIELD_WEAKENINGV2

//#define USE_LR_OBSERVER

/////////////////////Related to ANGLE ESTIMATION////////////////////////////////////////
#define INTERPOLATE_V7_ANGLE
#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_SENSORLESS
//#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_HALL
//#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_OPENLOOP
//#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_ENCODER
//#define DEFAULT_SENSOR_MODE MOTOR_SENSOR_MODE_HFI

#define DEFAULT_STARTUP_SENSOR  STARTUP_SENSOR_HALL //	STARTUP_SENSOR_HFI STARTUP_SENSOR_OPENLOOP,STARTUP_SENSOR_HALL,STARTUP_SENSOR_PWM_ENCODER,
#define HFI_VOLTAGE 1.0f
#define HFI_TEST_CURRENT 0.0f
#define HFI_THRESHOLD 0.0f //Defaults to 0.05Vbus if set to 0
#define DEFAULT_HFI_TYPE HFI_TYPE_NONE
//#define DEFAULT_HFI_TYPE HFI_TYPE_45
//#define DEFAULT_HFI_TYPE HFI_TYPE_D
//#define DEFAULT_HFI_TYPE HFI_TYPE_SPECIAL

//#define USE_HALL_START
#define HALL_VOLTAGE_THRESHOLD 2.0f


//#define USE_SPI_ENCODER //Only supports TLE5012B in SSC mode using onewire SPI on SPI3 F405...
#define ENCODER_E_OFFSET 22000

//#define USE_SALIENT_OBSERVER //If not defined, it assumes that Ld and Lq are equal, which is fine usually.

//#define SAFE_START_DEFAULT 0

// FIXME: Filler implementation, make work with Rust
#define getHallState(...) 0

#define LOGGING

#endif /* BOARD_BG_ETMAX_H */
