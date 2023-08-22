use hardware_mental_model::cache::cacheline::*;

use criterion::Criterion;
use criterion::BenchmarkId;
use criterion::criterion_main;
use criterion::criterion_group;

fn cacheline(c: &mut Criterion) {
    let mut group = c.benchmark_group("cacheline");

    group.bench_with_input(
        BenchmarkId::new("share", ""), 
        &(), 
        |b, _| {
        b.iter(|| share());
    });

    group.bench_with_input(
        BenchmarkId::new("false_share", ""), 
        &(), 
        |b, _| {
        b.iter(|| false_share());
    });

    group.bench_with_input(
        BenchmarkId::new("non_share", ""), 
        &(), 
        |b, _| {
        b.iter(|| non_share());
    });

    group.finish();
}

criterion_group!(benches, cacheline);
criterion_main!(benches);