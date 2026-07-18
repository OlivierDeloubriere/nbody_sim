use nbody_simulation::engine::Simulation;
use nbody_simulation::snapshot::Snapshot;

pub trait Steppable {
    fn step(&mut self, dt: f64);
    fn snapshot(&self) -> Snapshot;
}

impl Steppable for Simulation {
    fn step(&mut self, dt: f64) {
        Simulation::step(self, dt)
    }

    fn snapshot(&self) -> Snapshot {
        Simulation::snapshot(self)
    }
}

#[cfg(test)]
pub mod test_support {
    use super::*;

    pub struct FakeStepper {
        pub step_calls: u32,
        pub total_dt_stepped: f64,
        pub time: f64,
    }

    impl FakeStepper {
        pub fn new() -> Self {
            Self { step_calls: 0, total_dt_stepped: 0.0, time: 0.0 }
        }
    }

    impl Steppable for FakeStepper {
        fn step(&mut self, dt: f64) {
            self.step_calls += 1;
            self.total_dt_stepped += dt;
            self.time += dt;
        }

        fn snapshot(&self) -> Snapshot {
            Snapshot { time: self.time, positions: vec![], velocities: vec![] }
        }
    }
}