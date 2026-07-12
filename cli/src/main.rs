use nbody_simulation::{engine::Simulation};
use nbody_scenarios::solar_like_system;

fn main() {
    let g: f64 = 1.0;
    let softening: f64 = 1e-4;
    let dt: f64 = 1e-3;
    let n_steps: usize = 50_000;
    let report_every: usize = 5_000;

    let bodies = solar_like_system();
    let mut sim = Simulation::new(bodies, g, softening);

    let e0 = sim.total_energy();
    let l0 = sim.total_angular_momentum();

    println!("=== N-body simulation (naive O(n²) version) ===");
    println!("Bodies: {}", sim.bodies.len());
    println!("Initial energy:               {e0:.8e}");
    println!("Initial angular momentum (z): {:.8e}", l0.z);
    println!();

    for step in 1..=n_steps {
        sim.step(dt);

        if step % report_every == 0 {
            let e = sim.total_energy();
            let l = sim.total_angular_momentum();
            let drift_e = (e - e0) / e0;
            let drift_l = (l.z - l0.z) / l0.z;
            println!(
                "t = {:>7.2} | E = {:>12.6e} (drift {:>+8.4e}) | Lz = {:>12.6e} (drift {:>+8.4e})",
                sim.time, e, drift_e, l.z, drift_l
);
        }
    }

    println!();
    println!("Simulation complete. A drift on the order of 1e-3 or less");
    println!("in energy and angular momentum indicates a healthy integrator.");
}
