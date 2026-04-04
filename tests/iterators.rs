use matrix::M;

#[test]
fn test_matrix_row_iter() {
    let m = M!([[1.0_f32, 2.0, 3.0], [4.0, 5.0, 6.0]]);

    let rows: Vec<Vec<f32>> = m.row_iter().map(|row| row.to_vec()).collect();

    assert_eq!(rows, vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
}

#[test]
fn test_matrix_col_iter() {
    let m = M!([[1.0_f32, 2.0, 3.0], [4.0, 5.0, 6.0]]);

    let cols: Vec<Vec<f32>> = m
        .col_iter()
        .map(|col| col.copied().collect::<Vec<f32>>())
        .collect();

    assert_eq!(cols, vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
}

#[test]
fn test_matrix_row_iter_sum() {
    let m = M!([[1.0_f32, 2.0, 3.0], [4.0, 5.0, 6.0]]);

    assert_eq!(m.row_iter().sum(), 21.0);
}

#[test]
fn test_matrix_col_sum() {
    let m = M!([[1.0_f32, 2.0, 3.0], [4.0, 5.0, 6.0]]);

    let col_sums: Vec<f32> = m.col_iter().map(|col| col.sum()).collect();

    assert_eq!(col_sums, vec![5.0, 7.0, 9.0]);
}
