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

/// Demo scenario : a massive central body ("star") with
/// several light bodies in orbit ("planets") at different radii.
/// Simple to validate physically : each orbit must remain stable.
pub fn solar_like_system() -> Vec<Body> {
    let g: f64 = 1.0;
    let star_mass: f64 = 1.0;

    let mut bodies = vec![Body::new(star_mass, Vector3::ZERO, Vector3::ZERO)];

    // (orbital radius, relative mass)
    let planets = [(1.0, 1e-4), (2.0, 3e-4), (3.5, 1e-4)];

    for (r, mass) in planets {
        let v = (g * star_mass / r).sqrt(); // circular orbital velocity
        bodies.push(Body::new(
            mass,
            Vector3::new(r, 0.0, 0.0),
            Vector3::new(0.0, v, 0.0),
        ));
    }

    bodies
}

