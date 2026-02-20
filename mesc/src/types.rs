use crate::bindings::{
    motor_control_mode_e, motor_control_mode_e_MOTOR_CONTROL_MODE_DUTY,
    motor_control_mode_e_MOTOR_CONTROL_MODE_HANDBRAKE,
    motor_control_mode_e_MOTOR_CONTROL_MODE_MEASURING,
    motor_control_mode_e_MOTOR_CONTROL_MODE_POSITION,
    motor_control_mode_e_MOTOR_CONTROL_MODE_SPEED,
    motor_control_mode_e_MOTOR_CONTROL_MODE_TORQUE,
};

pub enum ControlMode {
    Torque,
    Speed,
    Duty,
    Position,
    Measuring,
    Handbrake,
}

impl Into<motor_control_mode_e> for ControlMode {
    fn into(self) -> motor_control_mode_e {
        match self {
            ControlMode::Torque => motor_control_mode_e_MOTOR_CONTROL_MODE_TORQUE,
            ControlMode::Speed => motor_control_mode_e_MOTOR_CONTROL_MODE_SPEED,
            ControlMode::Duty => motor_control_mode_e_MOTOR_CONTROL_MODE_DUTY,
            ControlMode::Position => motor_control_mode_e_MOTOR_CONTROL_MODE_POSITION,
            ControlMode::Measuring => motor_control_mode_e_MOTOR_CONTROL_MODE_MEASURING,
            ControlMode::Handbrake => motor_control_mode_e_MOTOR_CONTROL_MODE_HANDBRAKE,
        }
    }
}

pub enum MotorState {
    /// Starting up the inverter, we need to get the offsets for the current sensors and do
    /// any checks
    Initializing,

    /// PWM not generating output, but still running to trigger the ADC/check for
    /// hall sensors. Returned values from ADC used to detect if the motor is
    /// spinning, how fast, what speed...
    Detecting,

    /// Hold one phase at current
    Align,

    /// Measuring resistance and inducance of phase
    Measuring,

    /// Starting up in sensorless mode
    OpenLoopStartup,

    /// Checking motor is running synchronously and phaselocking
    OpenLoopTransition,

    /// Monitor the phase voltages while the PWM is disabled
    /// Perform Clark and park
    /// Run sensorless/hall observer to keep the angle
    /// Load the PID integral values with the current Vd and Vq
    Tracking,

    /// Run FOC modulation
    Run,

    /// Determine flux linkage
    GetKV,

    /// Variety of tests can be performed
    Test,
    /// Enter this state when the overcurrent or overvoltage trips, or illegal
    /// hall state or sensorless observer fault occurs. All PWM signals should be
    /// disabled, the timer may be in fault mode with all outputs disabled, or it
    /// may be required to implement the bit writes to turn off the outputs
    Error,

    /// After a fault state, or when no phase voltage sensors present, might want to
    /// implement a routine to restart the system on the fly - detect if motor is running,
    /// detect speed, phase, re-enable PWM
    Recovering,

    /// We are going to write all phases low, so current can continue to be read, and
    /// tristate the bridge when current exceeds the max setpoint on any phase.
    /// This will enable aggressive braking if spinning, with uncontrolled regeneration, but
    /// the primary usecase is low speed hold.
    SlamBrake,

    /// All PWM should be off state, nothing happening. Motor may be spinning freely
    Idle,
}
