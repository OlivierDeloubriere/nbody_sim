use crate::body::Body;
use crate::forces::compute_accelerations;
use crate::integrator::{Integrator, LeapfrogIntegrator};
use crate::snapshot::Snapshot;
use crate::vector3::Vector3;

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub g: f64,
    pub softening: f64,
    pub time: f64,
    integrator: Box<dyn Integrator>,
}

impl Simulation {
    pub fn new(bodies: Vec<Body>, g: f64, softening: f64) -> Self {
        Self::with_integrator(bodies, g, softening, Box::new(LeapfrogIntegrator))
    }

    pub fn with_integrator(
        bodies: Vec<Body>,
        g: f64,
        softening: f64,
        integrator: Box<dyn Integrator>,
    ) -> Self {
        Self { bodies, g, softening, time: 0.0, integrator }
    }

    pub fn compute_accelerations(&self) -> Vec<Vector3> {
        compute_accelerations(&self.bodies, self.g, self.softening)
    }

    pub fn step(&mut self, dt: f64) {
        self.integrator.step(&mut self.bodies, self.g, self.softening, dt);
        self.time += dt;
    }

    pub fn potential_energy(&self) -> f64 {
        let n = self.bodies.len();
        let mut energy = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = self.bodies[j].position - self.bodies[i].position;
                let dist = (delta.norm_squared() + self.softening * self.softening).sqrt();
                energy -= self.g * self.bodies[i].mass * self.bodies[j].mass / dist;
            }
        }
        energy
    }

    pub fn kinetic_energy(&self) -> f64 {
        self.bodies.iter().map(|b| b.kinetic_energy()).sum()
    }

    pub fn total_energy(&self) -> f64 {
        self.kinetic_energy() + self.potential_energy()
    }

    pub fn total_angular_momentum(&self) -> Vector3 {
        self.bodies
            .iter()
            .fold(Vector3::ZERO, |acc, b| acc + b.angular_momentum())
    }

    pub fn total_momentum(&self) -> Vector3 {
        self.bodies
            .iter()
            .fold(Vector3::ZERO, |acc, b| acc + b.momentum())
    }

    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            time: self.time,
            positions: self.bodies.iter().map(|b| b.position).collect(),
            velocities: self.bodies.iter().map(|b| b.velocity).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3::Vector3;

    #[test]
    fn test_energy_conservation_two_body() {
        let g: f64 = 1.0;
        let m1: f64 = 1.0;
        let m2: f64 = 1e-3;
        let r: f64 = 1.0;
        let v = (g * m1 / r).sqrt();

        let bodies = vec![
            Body::new(m1, Vector3::ZERO, Vector3::ZERO),
            Body::new(m2, Vector3::new(r, 0.0, 0.0), Vector3::new(0.0, v, 0.0)),
        ];

        let mut sim = Simulation::new(bodies, g, 1e-6);
        let e0 = sim.total_energy();

        let dt = 1e-3;
        for _ in 0..10_000 {
            sim.step(dt);
        }

        let e1 = sim.total_energy();
        let relative_drift = ((e1 - e0) / e0).abs();
        assert!(
            relative_drift < 1e-2,
            "dérive d'énergie trop importante: {relative_drift}"
        );
    }
}