use crate::body::Body;
use crate::forces::compute_accelerations;

pub trait Integrator {
    fn step(&self, bodies: &mut [Body], g: f64, softening: f64, dt: f64);
}

pub struct LeapfrogIntegrator;

impl Integrator for LeapfrogIntegrator {
    fn step(&self, bodies: &mut [Body], g: f64, softening: f64, dt: f64) {
        let acc = compute_accelerations(bodies, g, softening);
        for (body, a) in bodies.iter_mut().zip(acc.iter()) {
            body.velocity += *a * (dt * 0.5);
        }

        for body in bodies.iter_mut() {
            body.position += body.velocity * dt;
        }

        let acc = compute_accelerations(bodies, g, softening);
        for (body, a) in bodies.iter_mut().zip(acc.iter()) {
            body.velocity += *a * (dt * 0.5);
        }
    }
}