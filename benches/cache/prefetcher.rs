use hardware_mental_model::cache::prefetcher::*;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_main;
use criterion::criterion_group;

fn prefetcher(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("cache_prefetcher");
    for size in [KB, 2 * KB, 4 * KB, 6 * KB, 8 * KB].iter() {
        group.bench_with_input(
            BenchmarkId::new("row_major", size), 
            size, 
            |b, &size| {
            let mut arr = vec![vec![0; size]; size];
            b.iter(|| row_major_traversal(&mut arr) );
        });
        group.bench_with_input(
            BenchmarkId::new("column_major", size), 
            size, 
            |b, &size| {
            let mut arr = vec![vec![0; size]; size];
            b.iter(|| column_major_traversal(&mut arr));
        });
        group.bench_with_input(
            BenchmarkId::new("random_access", size), 
            size, 
            |b, &size| {
            let mut arr = vec![vec![0; size]; size];
            b.iter(|| random_access(&mut arr));
        });
    }

    group.finish();
}

criterion_group!(benches, prefetcher);
criterion_main!(benches);