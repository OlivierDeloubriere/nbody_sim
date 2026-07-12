//! Vecteur 3D minimal, sans dépendance externe.
//! On aurait pu utiliser `nalgebra` ou `glam`, mais pour ce projet
//! pédagogique, une implémentation maison garde le lien explicite
//! entre les formules physiques et le code.

use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Norme euclidienne au carré (évite un sqrt quand seule la comparaison compte).
    pub fn norm_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v.norm(), 5.0);
    }

    #[test]
    fn test_cross_orthogonal() {
        let x = Vector3::new(1.0, 0.0, 0.0);
        let y = Vector3::new(0.0, 1.0, 0.0);
        let z = x.cross(&y);
        assert_eq!(z, Vector3::new(0.0, 0.0, 1.0));
    }
}
