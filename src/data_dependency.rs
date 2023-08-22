pub fn dependent(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &Vec<i32>) {
    assert!(a.len() == 100000);
    assert!(b.len() == 100000);
    assert!(c.len() == 100000);

    for i in 0..=99998 {
        a[i] += b[i];
        b[i + 1] += c[i];
    }
    a[9999] += b[9999];
}

pub fn independent(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &Vec<i32>) {
    assert!(a.len() == 100000);
    assert!(b.len() == 100000);
    assert!(c.len() == 100000);

    a[0] += b[0];
    for i in 0..=99998 {
        b[i + 1] += c[i];
        a[i + 1] += b[i + 1];
    }
}
