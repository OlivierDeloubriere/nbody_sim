# Benchmarks

This file tracks the performance of each major implementation step, so the
progression (naive → Barnes-Hut → parallel) is documented with numbers, not
just claims.

## Methodology

Benchmarks are run with [`criterion`](https://github.com/bheisler/criterion.rs),
not a manual `Instant`-based timing loop. Criterion handles automatic
warm-up, runs enough iterations to get a statistically meaningful sample, and
reports a mean with a confidence interval rather than a single noisy number.
It also tracks the previous run and flags regressions/improvements
automatically, and produces HTML plots under `target/criterion/`.

- Metric: time to compute accelerations for all bodies once (`compute_accelerations`
  or its future equivalents), for `n` in `{100, 500, 1000, 2000, 5000}`.
- Bench target: `simulation/benches/forces.rs` (`cargo bench` always builds in
  release-equivalent optimized mode, no need to pass `--release`).
- Bodies are placed randomly in a cube, with random velocities and masses —
  this doesn't matter for the naive O(n²) method (every pair is computed
  regardless of layout), but it will matter once Barnes-Hut is introduced,
  since spatial clustering affects tree depth and traversal cost.
- Each row should record the machine it was run on. Numbers are not
  comparable across machines, only across rows measured on the *same* machine.
- `criterion::black_box` wraps inputs/outputs where needed to prevent the
  compiler from optimizing away a computation whose result is otherwise
  unused — worth checking if a result looks suspiciously fast.

Reproduce with:

```bash
cargo bench -p nbody_simulation
# HTML report: target/criterion/report/index.html
```

The table below reports the mean from the criterion summary. For the full
distribution and confidence interval of a given row, see the corresponding
HTML report (not reproduced here, since it's regenerated on every run).

## Results

| Version | Method | n=100 | n=500 | n=1000 | n=2000 | n=5000 | Machine |
|---|---|---|---|---|---|---|---|
| v0.1 (provisional) | Naive O(n²), single-threaded | 0.037 ms | 0.973 ms | 3.868 ms | 15.895 ms | 96.388 ms | Anthropic sandbox container — ad hoc `Instant` script, not criterion, reference only |
| v0.1 (criterion) | Naive O(n²), single-threaded | | | | | | *your machine — fill in with `cargo bench` output* |
| v0.2 | Barnes-Hut, single-threaded | | | | | | |
| v0.3 | Barnes-Hut + rayon | | | | | | |

**Note on the first row:** these numbers predate the switch to criterion —
measured in a generic sandbox container (unknown CPU, shared/virtualized
resources) with a simple `Instant`-based loop, not `cargo bench`. They're
useful only to confirm the expected O(n²) scaling (roughly ×4 time when n
doubles, which the numbers above do show), and are kept here for continuity
until superseded. Once you run `cargo bench -p nbody_simulation` on your own
machine, replace the second row with the real criterion output (mean ±
confidence interval) — that's the number that actually means something for
comparisons with Barnes-Hut later.

## Observations

- **v0.1 (naive):** scaling matches the expected O(n²) — doubling `n` roughly
  quadruples the time (500→1000: ×3.97; 1000→2000: ×4.11). This is the
  expected signature of direct pairwise summation and the baseline that
  Barnes-Hut is meant to improve on for large `n`.