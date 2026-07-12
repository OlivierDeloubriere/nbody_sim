// simulation/benches/forces.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbody_simulation::{body::Body, engine::Simulation, vector3::Vector3};

fn make_bodies(n: usize) -> Vec<Body> {
    // à toi de définir la génération (aléatoire ou déterministe)
    todo!()
}

fn bench_compute_accelerations(c: &mut Criterion) {
    for n in [100, 500, 1000, 2000, 5000] {
        let bodies = make_bodies(n);
        let sim = Simulation::new(bodies, 1.0, 1e-3);

        c.bench_function(&format!("compute_accelerations n={n}"), |b| {
            b.iter(|| sim.compute_accelerations())
        });
    }
}

criterion_group!(benches, bench_compute_accelerations);
criterion_main!(benches);