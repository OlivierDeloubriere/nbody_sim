use crate::steppable::Steppable;

pub struct FixedStepAccumulator {
    physics_dt: f64,
    sim_speed: f64,
    accumulator: f64,
    max_frame_time: f64,
}

impl FixedStepAccumulator {
    pub fn new(physics_dt: f64, sim_speed: f64) -> Self {
        Self {
            physics_dt,
            sim_speed,
            accumulator: 0.0,
            max_frame_time: 0.25,
        }
    }

    pub fn advance(&mut self, stepper: &mut impl Steppable, real_frame_time: f64) -> u32 {
        const EPSILON: f64 = 1e-9;

        let clamped = real_frame_time.min(self.max_frame_time);
        self.accumulator += clamped * self.sim_speed;

        let mut steps_taken = 0;
        while self.accumulator + EPSILON >= self.physics_dt {
            stepper.step(self.physics_dt);
            self.accumulator -= self.physics_dt;
            steps_taken += 1;
        }
        steps_taken
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::steppable::test_support::FakeStepper;

    #[test]
    fn advances_exactly_the_expected_number_of_steps() {
        let mut acc = FixedStepAccumulator::new(1e-3, 1.0);
        let mut stepper = FakeStepper::new();
        let steps = acc.advance(&mut stepper, 0.016);
        assert_eq!(steps, 16);
        assert_eq!(stepper.step_calls, 16);
    }

    #[test]
    fn carries_leftover_time_across_calls() {
        let mut acc = FixedStepAccumulator::new(1e-3, 1.0);
        let mut stepper = FakeStepper::new();
        acc.advance(&mut stepper, 0.0025);
        acc.advance(&mut stepper, 0.0025);
        assert_eq!(stepper.step_calls, 5);
    }

    #[test]
    fn clamps_huge_frame_stalls() {
        let mut acc = FixedStepAccumulator::new(1e-3, 1.0);
        let mut stepper = FakeStepper::new();
        let steps = acc.advance(&mut stepper, 10.0);
        assert_eq!(steps, 250);
    }

    #[test]
    fn sim_speed_scales_elapsed_time() {
        let mut acc = FixedStepAccumulator::new(1e-3, 2.0);
        let mut stepper = FakeStepper::new();
        let steps = acc.advance(&mut stepper, 0.008);
        assert_eq!(steps, 16);
    }
}