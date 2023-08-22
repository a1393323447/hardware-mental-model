use hardware_mental_model::cache::associativity::*;

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_main;
use criterion::criterion_group;

fn associativity(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_associativity");

    let steps = [
        1, 
        2, 
        4, 
        7, 8, 9,
        15, 16, 17,
        31, 32, 33,
        61, 64, 67,
        125, 128, 131,
        253, 256, 259,
        509, 512, 515,
        1019, 1024, 1031
    ];

    for step in steps {
        group.bench_with_input(
            BenchmarkId::new("step", step), 
            &step, 
            |b, &step| {
            let mut arr = vec![0; 1024 * 1024 * 16];
            b.iter(|| iter_with_step(&mut arr, step) );
        });
    }

    group.finish();
}

criterion_group!(benches, associativity);
criterion_main!(benches);