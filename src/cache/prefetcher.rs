pub fn row_major_traversal(arr: &mut Vec<Vec<usize>>) {
    let n = arr.len();
    for i in 0..n {
        assert!(arr[i].len() == n);
        let ri: usize = rand::random();
        std::intrinsics::black_box(ri);
        for j in 0..n {
            arr[i][j] += j;
        }
    }
}

pub fn column_major_traversal(arr: &mut Vec<Vec<usize>>) {
    let n = arr.len();
    for i in 0..n {
        assert!(arr[i].len() == n);
        let ri: usize = rand::random();
        std::intrinsics::black_box(ri);
        for j in 0..n {
            arr[j][i] += j;
        }
    }
}

pub fn random_access(arr: &mut Vec<Vec<usize>>) {
    let n = arr.len();
    for i in 0..n {
        assert!(arr[i].len() == n);
        for j in 0..n {
            let ri: usize = rand::random();
            arr[j][ri % n] += j;
        }
    }
}
