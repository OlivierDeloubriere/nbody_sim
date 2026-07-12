mod body;
mod simulation;
mod vector3;

use body::Body;
use simulation::Simulation;
use vector3::Vector3;

/// Scénario de démonstration : un corps massif central ("étoile") avec
/// quelques corps légers en orbite ("planètes") à différents rayons.
/// Simple à valider physiquement : chaque orbite doit rester stable.
fn build_solar_like_system() -> Vec<Body> {
    let g = 1.0;
    let star_mass = 1.0;

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

fn main() {
    let g = 1.0;
    let softening = 1e-4;
    let dt = 1e-3;
    let n_steps = 50_000;
    let report_every = 5_000;

    let bodies = build_solar_like_system();
    let mut sim = Simulation::new(bodies, g, softening);

    let e0 = sim.total_energy();
    let l0 = sim.total_angular_momentum();

    println!("=== Simulation N-corps (version naïve O(n²)) ===");
    println!("Corps: {}", sim.bodies.len());
    println!("Énergie initiale:            {e0:.8e}");
    println!("Moment angulaire initial (z): {:.8e}", l0.z);
    println!();

    for step in 1..=n_steps {
        sim.step(dt);

        if step % report_every == 0 {
            let e = sim.total_energy();
            let l = sim.total_angular_momentum();
            let drift_e = (e - e0) / e0;
            let drift_l = (l.z - l0.z) / l0.z;
            println!(
                "t = {:>7.2} | E = {:>12.6e} (dérive {:>+8.4e}) | Lz = {:>12.6e} (dérive {:>+8.4e})",
                sim.time, e, drift_e, l.z, drift_l
            );
        }
    }

    println!();
    println!("Simulation terminée. Une dérive de l'ordre de 1e-3 ou moins");
    println!("sur l'énergie et le moment angulaire indique un intégrateur sain.");
}
