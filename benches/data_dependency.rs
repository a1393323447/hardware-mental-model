use hardware_mental_model::data_dependency::*;

use criterion::Criterion;
use criterion::BenchmarkId;
use criterion::criterion_main;
use criterion::criterion_group;

pub fn gen() -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let a: Vec<i32> = (0..100000).map(|_| rand::random::<i32>()).collect();
    let b: Vec<i32> = (0..100000).map(|_| rand::random::<i32>()).collect();
    let c: Vec<i32> = (0..100000).map(|_| rand::random::<i32>()).collect();

    (a, b, c)
}

fn data_dependency(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_dependency");

    group.bench_with_input(
        BenchmarkId::new("dependent", ""), 
        &(),
        |bencher, _| {
            let (mut a, mut b, c) = gen();
            bencher.iter(move || {
                dependent(&mut a, &mut b, &c);
            });
        }
    );

    group.bench_with_input(
        BenchmarkId::new("independent", ""), 
        &(),
        |bencher, _| {
            let (mut a, mut b, c) = gen();
            bencher.iter(move || {
                independent(&mut a, &mut b, &c);
            });
        }
    );

    group.finish();
}

criterion_group!(benches, data_dependency);
criterion_main!(benches);