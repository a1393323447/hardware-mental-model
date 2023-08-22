use criterion::Criterion;
use criterion::BenchmarkId;
use criterion::criterion_main;
use criterion::criterion_group;
use rand::seq::SliceRandom;

pub trait Get {
    fn get(&self) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Foo {
    a: usize,
    b: isize,
    c: i32,
    e: u32,
}

impl Foo {
    pub fn new() -> Self {
        let mut v: Self = unsafe { std::mem::zeroed() };
        v.c = rand::random();
        v
    }
}

impl Get for Foo {
    fn get(&self) -> i32 {
        self.c
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Bar {
    e: u32,
    c: i32,
    b: isize,
    a: usize,
}

impl Bar {
    pub fn new() -> Self {
        let mut v: Self = unsafe { std::mem::zeroed() };
        v.c = rand::random();
        v
    }
}

impl Get for Bar {
    fn get(&self) -> i32 {
        self.c
    }
}

fn pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline");
    let mut arr: Vec<Box<dyn Get>> = vec![];

    for _ in 0..10000 {
        arr.push(Box::new(Foo::new()));
    }

    for _ in 0..10000 {
        arr.push(Box::new(Bar::new()));
    }

    group.bench_with_input(
        BenchmarkId::new("sorted", ""), 
        &arr, 
        |b, arr| {
        b.iter(|| {
            arr.iter().filter(|v| v.get() > 0).count()
        });
    });

    arr.shuffle(&mut rand::thread_rng());

    group.bench_with_input(
        BenchmarkId::new("unsorted", ""),
        &arr, 
        |b, arr| {
        b.iter(|| {
            arr.iter().filter(|v| v.get() > 0).count()
        });
    });

    group.finish();
}

criterion_group!(benches, pipeline);
criterion_main!(benches);