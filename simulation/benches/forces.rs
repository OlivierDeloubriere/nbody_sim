// simulation/benches/forces.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbody_simulation::engine::Simulation;
use nbody_scenarios::random_cluster;

fn bench_compute_accelerations(c: &mut Criterion) {
    for n in [100, 500, 1000, 2000, 5000] {
        let bodies = random_cluster(n, 42);
        let sim = Simulation::new(bodies, 1.0, 1e-3);

        c.bench_function(&format!("compute_accelerations n={n}"), |b| {
            b.iter(|| black_box(sim.compute_accelerations()))
        });
    }
}

criterion_group!(benches, bench_compute_accelerations);
criterion_main!(benches);