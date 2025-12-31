use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use euclidean_rhythm::euclidean;

fn benchmark_small_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_patterns");

    for (steps, pulses) in [(8, 3), (8, 5), (12, 5), (16, 7)] {
        group.bench_with_input(
            BenchmarkId::new("euclidean", format!("E({},{})", pulses, steps)),
            &(steps, pulses),
            |b, &(s, p)| {
                b.iter(|| euclidean(black_box(s), black_box(p), black_box(0)))
            },
        );
    }

    group.finish();
}

fn benchmark_medium_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("medium_patterns");

    for (steps, pulses) in [(32, 8), (32, 16), (64, 16), (64, 32)] {
        group.bench_with_input(
            BenchmarkId::new("euclidean", format!("E({},{})", pulses, steps)),
            &(steps, pulses),
            |b, &(s, p)| {
                b.iter(|| euclidean(black_box(s), black_box(p), black_box(0)))
            },
        );
    }

    group.finish();
}

fn benchmark_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");

    // Single pulse
    group.bench_function("E(1,16)", |b| {
        b.iter(|| euclidean(black_box(16), black_box(1), black_box(0)))
    });

    // Maximum density (n-1 pulses)
    group.bench_function("E(15,16)", |b| {
        b.iter(|| euclidean(black_box(16), black_box(15), black_box(0)))
    });

    // Half density
    group.bench_function("E(32,64)", |b| {
        b.iter(|| euclidean(black_box(64), black_box(32), black_box(0)))
    });

    group.finish();
}

fn benchmark_with_rotation(c: &mut Criterion) {
    let mut group = c.benchmark_group("with_rotation");

    // Test if rotation adds overhead
    group.bench_function("E(5,8)_no_rotation", |b| {
        b.iter(|| euclidean(black_box(8), black_box(5), black_box(0)))
    });

    group.bench_function("E(5,8)_rotation_2", |b| {
        b.iter(|| euclidean(black_box(8), black_box(5), black_box(2)))
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_small_patterns,
    benchmark_medium_patterns,
    benchmark_edge_cases,
    benchmark_with_rotation
);
criterion_main!(benches);
