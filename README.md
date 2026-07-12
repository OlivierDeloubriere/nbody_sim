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

```math
\vec{F}_{ij} = G \, m_i \, m_j \, \frac{\vec{r}_j - \vec{r}_i}{\left| \vec{r}_j - \vec{r}_i \right|^3}
```

and the total acceleration on body `i` is the sum over all other bodies:

```math
\vec{a}_i = G \sum_{j \neq i} m_j \, \frac{\vec{r}_j - \vec{r}_i}{\left| \vec{r}_j - \vec{r}_i \right|^3}
```

This is an O(n²) computation per timestep: every body interacts with every
other body. It's exact, but doesn't scale — this is the first bottleneck the
project is meant to address (see [Roadmap](#roadmap)).

### Softening

A raw `1/r²` force diverges as two bodies approach each other, which is both
physically unrealistic for point masses and numerically dangerous (it can
produce enormous accelerations that blow up the integration). The standard
fix is *softening*: replace $`|\vec{r}|`$ with $`\sqrt{|\vec{r}|^2 + \varepsilon^2}`$
for a small $`\varepsilon`$. This caps the force at close range at the cost
of a small, controlled inaccuracy. It's a numerical device, not a physical
effect.

### Why Leapfrog instead of explicit Euler

An explicit Euler integrator (`v += a*dt; x += v*dt`) is the obvious first
choice, but it is **not symplectic**: it does not conserve the energy of the
system over long integrations. In an orbital simulation this shows up as a
slow, systematic drift — orbits spiral in or out even though nothing physical
is causing that.

**Leapfrog** (kick-drift-kick) is used instead:

```math
\vec{v}\left(t + \frac{dt}{2}\right) = \vec{v}(t) + \vec{a}(t) \, \frac{dt}{2} \quad \text{(kick)}
```

```math
\vec{x}(t + dt) = \vec{x}(t) + \vec{v}\left(t + \frac{dt}{2}\right) dt \quad \text{(drift)}
```

```math
\vec{v}(t + dt) = \vec{v}\left(t + \frac{dt}{2}\right) + \vec{a}(t + dt) \, \frac{dt}{2} \quad \text{(kick)}
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

## Benchmarking methodology

Performance is measured with [`criterion`](https://github.com/bheisler/criterion.rs)
rather than a hand-rolled `Instant`-based timing loop. A manual chrono is easy
to write but easy to get subtly wrong — criterion exists specifically to
close those gaps:

- **Warm-up**: runs the code under measurement for a few seconds before
  recording anything, so CPU cache state and clock frequency have stabilized.
- **Statistically grounded results**: reports a mean with a confidence
  interval from hundreds of samples, not a single number that could be a
  lucky (or unlucky) run.
- **Outlier detection**: flags measurements that were likely perturbed by
  something external (OS scheduling, background load) instead of silently
  averaging them in.
- **Adaptive sampling**: detects when the default time budget isn't enough
  to collect a meaningful sample size for slower inputs, and warns rather
  than returning a degraded result quietly.
- **Protection against dead-code elimination**: `criterion::black_box`
  prevents the compiler from optimizing away a computation whose result is
  otherwise unused, which would otherwise make a benchmark measure nothing.
- **Regression tracking**: compares each run against the previous one and
  reports whether performance improved or regressed, which matters here
  specifically for validating that Barnes-Hut and parallelization actually
  deliver the expected gains, rather than trusting the implementation on faith.

Full methodology, raw results, and reproduction instructions live in
[`BENCHMARKS.md`](./BENCHMARKS.md).



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