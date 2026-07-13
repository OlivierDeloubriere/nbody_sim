use crate::body::Body;
use crate::snapshot::Snapshot;
use crate::vector3::Vector3;

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub g: f64,
    /// Paramètre d'adoucissement (softening) : évite la divergence de la force
    /// quand deux corps se rapprochent trop (singularité en 1/r²).
    /// C'est une astuce numérique standard en simulation N-corps,
    /// pas un phénomène physique réel.
    pub softening: f64,
    pub time: f64,
}

impl Simulation {
    pub fn new(bodies: Vec<Body>, g: f64, softening: f64) -> Self {
        Self { bodies, g, softening, time: 0.0 }
    }

    /// Calcul direct des accélérations, O(n²).
    /// Pour chaque corps i, on somme la contribution gravitationnelle
    /// de tous les autres corps j : a_i = G * sum_j m_j * (r_j - r_i) / |r_j - r_i + eps|^3
    pub fn compute_accelerations(&self) -> Vec<Vector3> {
        let n = self.bodies.len();
        let mut accelerations = vec![Vector3::ZERO; n];
        let eps2 = self.softening * self.softening;

        for i in 0..n {
            let mut acc = Vector3::ZERO;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let delta = self.bodies[j].position - self.bodies[i].position;
                let dist_sq = delta.norm_squared() + eps2;
                let dist = dist_sq.sqrt();
                // F = G * m_i * m_j / dist² ; a_i = F / m_i = G * m_j / dist²
                let factor = self.g * self.bodies[j].mass / (dist_sq * dist);
                acc += delta * factor;
            }
            accelerations[i] = acc;
        }
        accelerations
    }

    /// Un pas d'intégration Leapfrog (kick-drift-kick), symplectique.
    /// Contrairement à Euler explicite, cet intégrateur conserve
    /// approximativement l'énergie sur de longues durées, ce qui est
    /// crucial pour une simulation gravitationnelle crédible.
    pub fn step(&mut self, dt: f64) {
        let acc = self.compute_accelerations();

        // Kick (demi-pas sur la vitesse)
        for (body, a) in self.bodies.iter_mut().zip(acc.iter()) {
            body.velocity += *a * (dt * 0.5);
        }

        // Drift (pas complet sur la position)
        for body in self.bodies.iter_mut() {
            body.position += body.velocity * dt;
        }

        // Recalcul des accélérations à la nouvelle position
        let acc = self.compute_accelerations();

        // Kick (deuxième demi-pas sur la vitesse)
        for (body, a) in self.bodies.iter_mut().zip(acc.iter()) {
            body.velocity += *a * (dt * 0.5);
        }

        self.time += dt;
    }

    /// Énergie potentielle gravitationnelle totale du système.
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

    /// Système à deux corps en orbite circulaire : vérifie que
    /// l'énergie totale reste stable sur plusieurs orbites.
    #[test]
    fn test_energy_conservation_two_body() {
        let g: f64 = 1.0;
        let m1: f64 = 1.0;
        let m2: f64 = 1e-3; // corps léger en orbite autour d'un corps massif
        let r: f64 = 1.0;
        // Vitesse orbitale circulaire : v = sqrt(G * m1 / r)
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
