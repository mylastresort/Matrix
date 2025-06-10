use std::io::Write;

use matrix::projection;

use std::fs::File;

#[test]
fn test_projection() {
    let fov = 20.;
    let ratio = 3. / 2.;
    let near = 2.;
    let far = 50.;

    let mat = projection(fov, ratio, near, far);

    let mut file =
        File::create("matrix_display/proj").expect("Failed to create newline");

    for row in mat.data.chunks(mat.cols) {
        for i in 0..row.len() {
            write!(file, "{}", row[i]).expect("Failed to write newline");
            if i != row.len() - 1 {
                write!(file, ", ").expect("Failed to write newline");
            }
        }
        writeln!(file).expect("Failed to write newline");
    }
}
