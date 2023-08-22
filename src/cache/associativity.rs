use std::ops::AddAssign;

pub fn iter_with_step(arr: &mut Vec<usize>, step: usize) {
    let n = arr.len();
    let mut i = 0;
    for _ in 0..1000000 {
        unsafe { arr.get_unchecked_mut(i).add_assign(1); }
        i = (i + step) % n;
    }
}
