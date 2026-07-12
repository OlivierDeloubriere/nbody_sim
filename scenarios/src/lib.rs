use nbody_simulation::{body::Body, vector3::Vector3};
use rand::{Rng, SeedableRng};

pub fn random_cluster(n: usize, seed: u64) -> Vec<Body> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..n)
        .map(|_| {
            Body::new(
                rng.gen_range(0.5..2.0),
                Vector3::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)),
                Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            )
        })
        .collect()
}

/// Scénario de démonstration : un corps massif central ("étoile") avec
/// quelques corps légers en orbite ("planètes") à différents rayons.
/// Simple à valider physiquement : chaque orbite doit rester stable.
pub fn solar_like_system() -> Vec<Body> {
    let g: f64 = 1.0;
    let star_mass: f64 = 1.0;

    let mut bodies = vec![Body::new(star_mass, Vector3::ZERO, Vector3::ZERO)];

    // (rayon orbital, masse relative)
    let planets = [(1.0, 1e-4), (2.0, 3e-4), (3.5, 1e-4)];

    for (r, mass) in planets {
        let v = (g * star_mass / r).sqrt(); // vitesse orbitale circulaire
        bodies.push(Body::new(
            mass,
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v, 0.0),
        ));
    }

    bodies
}

