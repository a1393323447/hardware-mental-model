pub fn row_major_traversal(arr: &mut Vec<Vec<usize>>) {
    let n = arr.len();
    for i in 0..n {
        assert!(arr[i].len() == n);
        for j in 0..n {
            arr[i][j] += j;
        }
    }
}

pub fn column_major_traversal(arr: &mut Vec<Vec<usize>>) {
    let n = arr.len();
    for i in 0..n {
        assert!(arr[i].len() == n);
        for j in 0..n {
            arr[j][i] += j;
        }
    }
}
