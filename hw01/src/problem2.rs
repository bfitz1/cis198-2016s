/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let (r1, c1) = (mat1[0].len(), mat1.len());
    let (r2, c2) = (mat2[0].len(), mat2.len());

    assert!(c1 == r2);

    let mut mat = Vec::new();
    for r in 0..r1 {
        let mut row = Vec::new();
        for c in 0..c2 {
            let mut dot = 0.0;
            for i in 0..c1 {
                dot += mat1[r][i] * mat2[i][c];
            }
            row.push(dot);
        }
        mat.push(row);
    }
    mat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_matrix() {
        let mut mat1 = vec![vec![0.; 3]; 3];
        for i in 0..mat1.len() {
            mat1[i][i] = 1.;
        }
        let mat2 = vec![vec![5. ;3]; 3];
        let result = mat_mult(&mat1, &mat2);
        for i in 0..result.len() {
            for j in 0..result[i].len() {
                assert_eq!(result[i][j], mat2[i][j]);
            }
        }
    }
}