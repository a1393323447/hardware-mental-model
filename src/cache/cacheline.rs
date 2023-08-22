use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn increase(v: &AtomicUsize) {
    for _ in 0..10000 {
        v.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn share() {
    let v = AtomicUsize::new(0);
    thread::scope(|s| {
        let ta = s.spawn(|| increase(&v));
        let tb = s.spawn(|| increase(&v));
        let tc = s.spawn(|| increase(&v));
        let td = s.spawn(|| increase(&v));

        ta.join().unwrap();
        tb.join().unwrap();
        tc.join().unwrap();
        td.join().unwrap();
    });
}

pub fn false_share() {
    let a = AtomicUsize::new(0);
    let b = AtomicUsize::new(0);
    let c = AtomicUsize::new(0);
    let d = AtomicUsize::new(0);

    thread::scope(|s| {
        let ta = s.spawn(|| increase(&a));
        let tb = s.spawn(|| increase(&b));
        let tc = s.spawn(|| increase(&c));
        let td = s.spawn(|| increase(&d));

        ta.join().unwrap();
        tb.join().unwrap();
        tc.join().unwrap();
        td.join().unwrap();
    });
}

fn increase_2(v: &AlignAtomicUsize) {
    for _ in 0..10000 {
        v.v.fetch_add(1, Ordering::Relaxed);
    }
}

#[repr(align(64))]
struct AlignAtomicUsize {
    v: AtomicUsize,
}

impl AlignAtomicUsize {
    pub fn new(val: usize) -> Self {
        Self { v: AtomicUsize::new(val) }
    }
}

pub fn non_share() {
    let a = AlignAtomicUsize::new(0);
    let b = AlignAtomicUsize::new(0);
    let c = AlignAtomicUsize::new(0);
    let d = AlignAtomicUsize::new(0);

    thread::scope(|s| {
        let ta = s.spawn(|| increase_2(&a));
        let tb = s.spawn(|| increase_2(&b));
        let tc = s.spawn(|| increase_2(&c));
        let td = s.spawn(|| increase_2(&d));

        ta.join().unwrap();
        tb.join().unwrap();
        tc.join().unwrap();
        td.join().unwrap();
    });
}
