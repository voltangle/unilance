/*
 **
 ******************************************************************************
 * @file           : MESChw_setup.h
 * @brief          : Initialisation code for the PCB
 ******************************************************************************
 * @attention
 *
 * <h2><center>&copy; Copyright (c) 2020 David Molony.
 * All rights reserved.</center></h2>
 *
 * This software component is licensed under BSD 3-Clause license,
 * the "License"; You may not use this file except in compliance with the
 * License. You may obtain a copy of the License at:
 *                        opensource.org/licenses/BSD-3-Clause
 *
 ******************************************************************************

 * MESChw_setup.c
 *
 *  Created on: 25 Jul 2020
 *      Author: David Molony
 */

#if !defined(MESChw_setup_H)
#define MESChw_setup_H

#include "MESCfoc.h"

#define FLASH_STORAGE_PAGE 7

/////////////END USER DEFINES//////////////////////

// _OR_
// void hw_setup_init( hw_setp_s * hw_setup );

typedef struct {
    hardware_vars_t Rphase;    // float containing phase resistance in mOhms,
                               // populated by MEASURING if not already known;
    hardware_vars_t Lphase;    // float containing phase inductance in uH,
    hardware_vars_t Lqphase;   // range from very very low inductance high kV strong
                               // magnet BLDC motors to low kV weak magnet ones;
    hardware_vars_t Lqd_diff;  // Lq-Ld for using MTPA
    uint8_t uncertainty;
    float motor_flux;
    float measure_current;
    float measure_voltage;
} motor_s;

/*
Hardware-specific implementation

The following function prototypes must be defined in the corresponding:
    MESC_Fxxxx/Core/Src/MESChw_setup.c

in addition to pre-processor defines in the corresponding:
    MESC_Fxxxx/Core/Inc/mesc_hal.h
*/

/*
Hardware identifiers

#define MESC_GPIO_HALL
*/

/*
Function prototypes
*/

void hw_init(
    MESC_motor_typedef* _motor);  // Fills the parameters of the hardware struct,
                                  // simplifies some into useful overall gain values

void setAWDVals();
void MESCfoc_getRawADC(MESC_motor_typedef* _motor);
void MESCfoc_getRawADCVph(MESC_motor_typedef* _motor);

int MESC_getHallState(void);

void mesc_init_1(MESC_motor_typedef* _motor);  // Perform HW specific initialisation for
                                               // MESCInit() before delay
void mesc_init_2(MESC_motor_typedef* _motor);  // Perform HW specific initialisation for
                                               // MESCInit() after delay
void mesc_init_3(MESC_motor_typedef* _motor);  // Perform HW specific initialisation for
                                               // MESCInit() after hw_init()

/*
Profile defaults

Temperature parameters
MESC_TEMP_MOS_R_F
MESC_TEMP_MOS_METHOD
MESC_TEMP_MOS_SCHEMA
MESC_TEMP_MOS_SH_BETA
MESC_TEMP_MOS_SH_R
MESC_TEMP_MOS_SH_R0

MESC_TEMP_MOTOR_R_F
MESC_TEMP_MOTOR_METHOD
MESC_TEMP_MOTOR_SCHEMA
MESC_TEMP_MOTOR_SH_BETA
MESC_TEMP_MOTOR_SH_R
MESC_TEMP_MOTOR_SH_R0
*/

#endif
