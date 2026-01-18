use crate::{balance::{BalanceState, HysteresisState, RideAssistState}, imu::IMUData};
use micromath::F32Ext;

impl BalanceState {
    /// Call this right after setting up the config.
    pub fn init(&mut self) {
        self.setpoint = self.config.setpoint_zero;
        let dt_sec = self.config.dt as f32 / 1000000.0;
    }

    /// Main balance loop function. Has to be called in ISR, for real time guarantees.
    pub fn iterate(&mut self, imu_state: IMUData) -> f32 {
        let pid_out = self.iterate_pid(&imu_state);
        if self.config.rideassist.enable {
            self.iterate_ride_assist(&imu_state, pid_out);
        }
        return pid_out;
    }

    fn iterate_pid(&mut self, imu_state: &IMUData) -> f32 {
        let angle_error = imu_state.pitch - self.setpoint;

        let p_term = self.config.kp as f32 * angle_error.powf(self.config.kp_expo as f32);

        self.integral_accum += self.config.ki as f32 * angle_error * self.dt_secs;
        // keep the integrator in sane limits
        self.integral_accum = self
            .integral_accum
            .clamp(self.config.integral_min, self.config.integral_max);

        let d_term = if imu_state.pitch_rate.is_sign_positive() {
            self.config.kd_forward as f32 * imu_state.pitch_rate
        } else {
            self.config.kd_backward as f32 * imu_state.pitch_rate
        };

        let temp_out = p_term + self.integral_accum + d_term;

        // Anti-windup
        if temp_out > self.config.out_max as f32 && angle_error > self.setpoint
            || temp_out < self.config.out_min as f32 && angle_error < self.setpoint
        {
            self.integral_accum -= self.config.ki as f32 * angle_error * self.dt_secs;
        }

        return p_term + self.integral_accum + d_term;
    }

    /// Doesn't output anything, because its output is actually the internal setpoint.
    fn iterate_ride_assist(&mut self, imu_state: &IMUData, current_out: f32) {
        // Check for current state and if any changes are in order

        match self.rideassist_state {
            RideAssistState::Acceleration => todo!(),
            RideAssistState::Idle => {
                let upper_threshold =
                    self.config.rideassist.acceleration_state_threshold
                        + self.config.rideassist.state_hysteresis as f32;
                let lower_threshold = self.config.rideassist.braking_state_threshold
                    - self.config.rideassist.state_hysteresis as f32;

                if current_out > upper_threshold {
                    self.rideassist_prev_state = self.rideassist_state;
                    self.rideassist_state = RideAssistState::Acceleration;
                }
                if current_out < lower_threshold {
                    self.rideassist_prev_state = self.rideassist_state;
                    self.rideassist_state = RideAssistState::Braking;
                }
            }
            RideAssistState::Braking => todo!(),
        }
    }

    fn rideassist_switch_states(new_state: RideAssistState) {

    }
}
