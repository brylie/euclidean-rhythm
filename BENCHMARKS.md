# Performance Benchmarks

Baseline performance measurements for the Euclidean rhythm library (v0.1.0).

## Environment
- Rust: 1.92.0
- Edition: 2024
- Criterion: 0.5

## Results Summary

### Small Patterns (Typical Musical Use)
Pattern sizes commonly used in music (8-16 steps):

| Pattern | Time | Use Case |
|---------|------|----------|
| E(3,8) | ~750 ns | Cuban tresillo |
| E(5,8) | ~700 ns | West African bell |
| E(5,12) | ~1.16 µs | Persian rhythm |
| E(7,16) | ~1.48 µs | Bossa nova |

**Key Insight**: All typical musical patterns generate in **under 2 microseconds** - fast enough for real-time music applications.

### Medium Patterns
For more complex rhythms (32-64 steps):

| Pattern | Time |
|---------|------|
| E(8,32) | ~2.76 µs |
| E(16,32) | ~2.69 µs |
| E(16,64) | ~5.13 µs |
| E(32,64) | ~5.13 µs |

**Key Insight**: Even 64-step patterns generate in **~5 microseconds** - still negligible overhead.

### Edge Cases
Special patterns that test algorithm efficiency:

| Pattern | Time | Notes |
|---------|------|-------|
| E(1,16) | ~162 ns | Single pulse (fast path) |
| E(15,16) | ~1.06 µs | Maximum density |
| E(32,64) | ~5.13 µs | Half density |

### Rotation Overhead
Testing the cost of pattern rotation:

| Operation | Time | Overhead |
|-----------|------|----------|
| E(5,8) no rotation | ~698 ns | baseline |
| E(5,8) rotation=2 | ~709 ns | +11 ns (~1.6%) |

**Key Insight**: Rotation adds **negligible overhead** (~10-20ns).

## Performance Conclusions

1. **Current implementation is fast enough** for all musical use cases
   - Generates patterns in microseconds, not milliseconds
   - Can generate thousands of patterns per second

2. **No optimization needed for v0.1.0**
   - Performance is already excellent
   - Readability and correctness more valuable than marginal gains

3. **Future optimization targets** (if needed in v0.2.0+):
   - Large patterns (>100 steps) could benefit from Vec<Vec> elimination
   - Batch generation of multiple patterns
   - SIMD for rotation operations

## Running Benchmarks

```bash
cargo bench --bench euclidean_bench
```

To compare against future changes:
```bash
cargo bench --bench euclidean_bench -- --save-baseline v0.1.0
```

## Benchmark Details

Benchmarks measure:
- **Small patterns**: Common musical rhythms (8-16 steps)
- **Medium patterns**: Complex polyrhythms (32-64 steps)
- **Edge cases**: Single pulse, maximum density, half density
- **Rotation overhead**: Impact of pattern rotation

Each benchmark:
- Runs 100 samples
- Automatically handles warm-up
- Reports outliers
- Calculates statistical significance
