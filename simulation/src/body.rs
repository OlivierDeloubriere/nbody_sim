use crate::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Body {
    pub mass: f64,
    pub position: Vector3,
    pub velocity: Vector3,
}

impl Body {
    pub fn new(mass: f64, position: Vector3, velocity: Vector3) -> Self {
        Self { mass, position, velocity }
    }

    pub fn kinetic_energy(&self) -> f64 {
        0.5 * self.mass * self.velocity.norm_squared()
    }

    pub fn momentum(&self) -> Vector3 {
        self.velocity * self.mass
    }

    pub fn angular_momentum(&self) -> Vector3 {
        // L = r × p
        self.position.cross(&self.momentum())
    }
}
