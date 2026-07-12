# Benchmarks

This file tracks the performance of each major implementation step, so the
progression (naive → Barnes-Hut → parallel) is documented with numbers, not
just claims.

## Methodology

- Metric: time to compute accelerations for all bodies once (`compute_accelerations`
  or its future equivalents), averaged over several repetitions.
- Build: `cargo build --release` (unoptimized/debug numbers are not meaningful here).
- Bodies are placed randomly in a cube, with random velocities and masses —
  this doesn't matter for the naive O(n²) method (every pair is computed
  regardless of layout), but it will matter once Barnes-Hut is introduced,
  since spatial clustering affects tree depth and traversal cost.
- Each row should record the machine it was run on. Numbers are not
  comparable across machines, only across rows measured on the *same* machine.

Reproduce with:

```bash
cargo build --release
# then run whatever benchmark binary/harness corresponds to that row
```

*(Once `criterion` is added — see project roadmap — this should be replaced
by `cargo bench` output, which handles warm-up and statistical noise properly.
The numbers below come from a simple `Instant`-based timing loop, which is
good enough for order-of-magnitude comparisons but not for rigorous
micro-benchmarking.)*

## Results

| Version | Method | n=100 | n=500 | n=1000 | n=2000 | n=5000 | Machine |
|---|---|---|---|---|---|---|---|
| v0.1 | Naive O(n²), single-threaded | 0.037 ms | 0.973 ms | 3.868 ms | 15.895 ms | 96.388 ms | Anthropic sandbox container (reference only — see note) |
| v0.1 | Naive O(n²), single-threaded | | | | | | *your machine — fill in* |
| v0.2 | Barnes-Hut, single-threaded | | | | | | |
| v0.3 | Barnes-Hut + rayon | | | | | | |

**Note on the first row:** these numbers were measured in a generic sandbox
container (unknown CPU, shared/virtualized resources) — they're useful only
to confirm the expected O(n²) scaling (roughly ×4 time when n doubles, which
the numbers above do show), not as a real performance baseline. The row right
below it is where your own measurements on your actual development machine
(WSL2, Rust 1.97) should go — that's the number that actually means something
for comparisons later.

## Observations

- **v0.1 (naive):** scaling matches the expected O(n²) — doubling `n` roughly
  quadruples the time (500→1000: ×3.97; 1000→2000: ×4.11). This is the
  expected signature of direct pairwise summation and the baseline that
  Barnes-Hut is meant to improve on for large `n`.