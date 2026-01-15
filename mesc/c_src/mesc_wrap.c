#include "mesc_wrap.h"

// basically the "C main function"
void ul_mesc_start(MESC_motor_typedef *motor) {
    motor_init(motor);
    MESCfoc_Init(motor);
}
