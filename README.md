# N-Body Gravitational Simulation

A gravitational N-body simulation in Rust, built to connect classical mechanics
theory to a performant, tested implementation. This project started as a naive
O(n²) direct-summation simulator and is meant to evolve toward a Barnes-Hut
tree code with parallelization.

## Why this project

Most of my background is in theoretical physics (PhD) and distributed systems
engineering in Rust, but not in applied numerical modeling. This project is a
deliberate attempt to close that gap: take a well-understood physical system,
implement it correctly, and be explicit about the numerical choices that make
the difference between a simulation that "looks plausible" and one that is
actually trustworthy.

## Physics background

### The N-body problem

For `n` massive bodies interacting under Newtonian gravity, the force on body
`i` from body `j` is:

```
F_ij = G * m_i * m_j * (r_j - r_i) / |r_j - r_i|^3
```

and the total acceleration on body `i` is the sum over all other bodies:

```
a_i = G * sum_{j != i} m_j * (r_j - r_i) / |r_j - r_i|^3
```

This is an O(n²) computation per timestep: every body interacts with every
other body. It's exact, but doesn't scale — this is the first bottleneck the
project is meant to address (see [Roadmap](#roadmap)).

### Softening

A raw `1/r²` force diverges as two bodies approach each other, which is both
physically unrealistic for point masses and numerically dangerous (it can
produce enormous accelerations that blow up the integration). The standard
fix is *softening*: replace `|r|` with `sqrt(|r|² + ε²)` for a small `ε`. This
caps the force at close range at the cost of a small, controlled inaccuracy.
It's a numerical device, not a physical effect.

### Why Leapfrog instead of explicit Euler

An explicit Euler integrator (`v += a*dt; x += v*dt`) is the obvious first
choice, but it is **not symplectic**: it does not conserve the energy of the
system over long integrations. In an orbital simulation this shows up as a
slow, systematic drift — orbits spiral in or out even though nothing physical
is causing that.

**Leapfrog** (kick-drift-kick) is used instead:

```
v(t + dt/2) = v(t) + a(t) * dt/2       # kick
x(t + dt)   = x(t) + v(t + dt/2) * dt  # drift
v(t + dt)   = v(t + dt/2) + a(t+dt) * dt/2  # kick
```

Leapfrog is symplectic: it doesn't conserve energy exactly at every step, but
it conserves a quantity *close to* the true energy over arbitrarily long
integrations, with bounded oscillation rather than systematic drift. This is
the standard choice in N-body astrophysics codes for exactly this reason.

### Validation, not just implementation

Conservation of total energy and total angular momentum are used as
correctness checks, not just diagnostics. If either drifts significantly over
a run, that's a signal of a bug or an unstable timestep — not just "numerical
noise." See [Results](#results) for what a healthy run looks like.

## Architecture

```
src/
├── vector3.rs    # Minimal 3D vector type (no external dependency)
├── body.rs       # Body: mass, position, velocity + per-body diagnostics
├── simulation.rs # Force computation, Leapfrog integrator, system diagnostics
└── main.rs       # Demo scenario: a star with orbiting bodies
```

The vector math is hand-rolled rather than pulled from a crate like `nalgebra`
or `glam`, on purpose: for a project whose point is to show the link between
equations and code, an opaque external type would obscure exactly the part
that matters.

`Simulation` owns the physical state (`Vec<Body>`, `G`, softening) and exposes:

- `compute_accelerations()` — O(n²) direct summation
- `step(dt)` — one Leapfrog integration step
- `total_energy()`, `total_angular_momentum()`, `total_momentum()` — system-level
  diagnostics used both in the demo output and in the test suite

## Usage

```bash
cargo build --release
cargo test              # includes a physical correctness test, not just unit tests
./target/release/nbody_sim
```

The demo scenario (`build_solar_like_system`) sets up a massive central body
with several lighter bodies on circular orbits at different radii, and prints
energy/angular momentum drift at regular intervals.

## Results

Running the default scenario (4 bodies, `dt = 1e-3`, 50,000 steps) gives:

```
Énergie initiale:            -1.39339714e-4
Moment angulaire initial (z): 7.11346938e-4

t =    5.00 | E drift ≈ 8.6e-11  | Lz drift ≈ -1.7e-15
...
t =   50.00 | E drift ≈ -8.0e-11 | Lz drift ≈ -1.0e-14
```

Energy and angular momentum stay stable to roughly 10 significant figures
over the full run — well within the tolerance used in the automated test
(`test_energy_conservation_two_body`, which asserts drift below 1%). This is
the kind of result that distinguishes "the simulation runs and produces
pictures" from "the simulation is physically trustworthy."

## Roadmap

- [ ] **Euler vs. Leapfrog comparison** — add an explicit Euler integrator
      and demonstrate the energy drift it produces on the same scenario, to
      make the case for Leapfrog concrete rather than asserted.
- [ ] **Barnes-Hut approximation** — replace O(n²) force summation with an
      octree-based approximation (O(n log n)) for larger body counts.
- [ ] **Parallelization** — parallelize force computation with `rayon`.
- [ ] **Benchmarks** — compare naive vs. Barnes-Hut, sequential vs. parallel,
      with `criterion`.
- [ ] **Visualization** — basic trajectory plotting (likely via a Python
      post-processing script reading simulation output, to keep the Rust core
      focused on the physics/performance work).

## License

MIT (or adjust to your preference).