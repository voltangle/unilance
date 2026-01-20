use crate::{
    balance::{BalanceState, RideAssistCoreState},
    imu::IMUData,
};
use micromath::F32Ext;

impl BalanceState {
    /// Call this right after setting up the config.
    pub fn init(&mut self) {
        self.setpoint = self.run_config.setpoint_zero;
        self.dt_sec = self.config.dt as f32 / 1000000.0;
    }

    /// Main balance loop function. Has to be called in ISR, for real time guarantees.
    pub fn iterate(&mut self, imu_state: IMUData) -> f32 {
        let pid_out = self.iterate_pid(&imu_state);
        if self.run_config.rideassist.enable {
            self.iterate_ride_assist(&imu_state, pid_out);
        }
        return pid_out;
    }

    fn iterate_pid(&mut self, pos_state: &IMUData) -> f32 {
        let angle_error = pos_state.pitch - self.setpoint;

        let p_term = self.config.kp as f32 * angle_error.powf(self.config.kp_expo as f32);

        self.pid_integral_accum += self.config.ki as f32 * angle_error * self.dt_sec;
        // keep the integrator in sane limits
        self.pid_integral_accum = self
            .pid_integral_accum
            .clamp(self.config.integral_min, self.config.integral_max);

        let d_term = if pos_state.pitch_rate.is_sign_positive() {
            self.config.kd_forward as f32 * pos_state.pitch_rate
        } else {
            self.config.kd_backward as f32 * pos_state.pitch_rate
        };

        let temp_out = p_term + self.pid_integral_accum + d_term;

        // Anti-windup
        if temp_out > self.config.out_max as f32 && angle_error > self.setpoint
            || temp_out < self.config.out_min as f32 && angle_error < self.setpoint
        {
            self.pid_integral_accum -= self.config.ki as f32 * angle_error * self.dt_sec;
        }

        return p_term + self.pid_integral_accum + d_term;
    }

    /// Doesn't output anything, because its output is actually the internal setpoint.
    fn iterate_ride_assist(&mut self, pos_state: &IMUData, current_out: f32) {
        // Check for current state and if any changes are in order
        match self.rideassist.state {
            RideAssistCoreState::Acceleration => {
                let threshold_idle = self.config.rideassist.accel_state_threshold
                    - self.config.rideassist.state_hysteresis as f32;
                let threshold_braking = self.config.rideassist.braking_state_threshold
                    - self.config.rideassist.state_hysteresis as f32;

                // Allow to skip idle and go directly to braking, although no idea if that
                // will ever happen
                if pos_state.x_accel < threshold_braking {
                    self.rideassist_switch_states(RideAssistCoreState::Braking);
                } else if pos_state.x_accel < threshold_idle {
                    self.rideassist_switch_states(RideAssistCoreState::Idle);
                }
            }
            RideAssistCoreState::Idle => {
                let upper_threshold = self.config.rideassist.accel_state_threshold
                    + self.config.rideassist.state_hysteresis as f32;
                let lower_threshold = self.config.rideassist.braking_state_threshold
                    - self.config.rideassist.state_hysteresis as f32;

                if pos_state.x_accel > upper_threshold {
                    self.rideassist_switch_states(RideAssistCoreState::Acceleration);
                }
                if pos_state.x_accel < lower_threshold {
                    self.rideassist_switch_states(RideAssistCoreState::Braking);
                }
            }
            RideAssistCoreState::Braking => {
                let threshold_idle = self.config.rideassist.braking_state_threshold
                    + self.config.rideassist.state_hysteresis as f32;
                let threshold_accel = self.config.rideassist.accel_state_threshold
                    + self.config.rideassist.state_hysteresis as f32;

                // Allow to skip idle and go directly to acceleration, although no idea if
                // that will ever happen
                if pos_state.x_accel > threshold_accel {
                    self.rideassist_switch_states(RideAssistCoreState::Acceleration);
                } else if pos_state.x_accel > threshold_idle {
                    self.rideassist_switch_states(RideAssistCoreState::Idle);
                }
            }
        }

        // And now actually execute the iteration
        match self.rideassist.state {
            RideAssistCoreState::Acceleration => todo!(),
            RideAssistCoreState::Idle => todo!(),
            RideAssistCoreState::Braking => todo!(),
        }
    }

    // Switches states with gating, aka will not switch to the same state twice.
    fn rideassist_switch_states(&mut self, new_state: RideAssistCoreState) {
        match self.rideassist.state {
            RideAssistCoreState::Acceleration => {
                if new_state == RideAssistCoreState::Idle
                    || new_state == RideAssistCoreState::Braking
                {
                    self.rideassist.prev_state = self.rideassist.state;
                    self.rideassist.state = new_state;
                }
            }
            RideAssistCoreState::Idle => {
                if new_state == RideAssistCoreState::Acceleration
                    || new_state == RideAssistCoreState::Braking
                {
                    self.rideassist.prev_state = self.rideassist.state;
                    self.rideassist.state = new_state;
                }
            }
            RideAssistCoreState::Braking => {
                if new_state == RideAssistCoreState::Idle
                    || new_state == RideAssistCoreState::Acceleration
                {
                    self.rideassist.prev_state = self.rideassist.state;
                    self.rideassist.state = new_state;
                }
            }
        }
    }
}
