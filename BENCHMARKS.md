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
| v0.1 (criterion) | Naive O(n²), single-threaded | 0.0594 ms [0.0586, 0.0603] | 1.553 ms [1.534, 1.572] | 6.073 ms [5.982, 6.170] | 24.359 ms [24.106, 24.622] | 151.31 ms [150.16, 152.48] | WSL2 (Ubuntu), Rust 1.97, Olivier's dev machine |
| v0.2 | Barnes-Hut, single-threaded | | | | | | |
| v0.3 | Barnes-Hut + rayon | | | | | | |

**Note on the first row:** these numbers predate the switch to criterion —
measured in a generic sandbox container (unknown CPU, shared/virtualized
resources) with a simple `Instant`-based loop, not `cargo bench`. They're
kept here only as a historical sanity check on the O(n²) scaling shape;
the second row (real criterion measurements, on an actual dev machine) is
the one that matters for any real comparison going forward — in particular
once Barnes-Hut (v0.2) is implemented, it should be compared against the
v0.1 (criterion) row, not the provisional one.

## Observations

- **v0.1 (provisional, sandbox):** scaling matches the expected O(n²) —
  doubling `n` roughly quadruples the time (500→1000: ×3.97; 1000→2000:
  ×4.11).
- **v0.1 (criterion, dev machine):** confirms the same O(n²) signature with
  tighter, statistically grounded measurements — 500→1000: ×3.91;
  1000→2000: ×4.01; 2000→5000 (×2.5 in n): ×6.21 (expected ×6.25 for pure
  O(n²), essentially exact). This is the expected signature of direct
  pairwise summation and the baseline that Barnes-Hut (v0.2) is meant to
  improve on for large `n` — the O(n log n) complexity should show a
  markedly sub-quadratic growth curve by comparison.