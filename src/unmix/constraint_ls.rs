use super::unconstraint_ls::unmix;
use super::util::ravel;
use nalgebra::{DMatrix, DVector};

pub fn unmixing_constraint(pixel: &Vec<f64>, signatures: &Vec<Vec<f64>>) -> Vec<f64> {
    let abundonces = unmix(pixel, signatures);

    let abundonces = DVector::from_vec(abundonces);

    let nrows = signatures.len();
    let ncols = signatures[0].len();

    let signatures =
        DMatrix::from_row_slice(nrows, ncols, ravel(&signatures).as_slice()).transpose();

    let inverse = (&signatures.transpose() * &signatures)
        .pseudo_inverse(1e-12)
        .unwrap();

    let z = DVector::from_vec(vec![1.; nrows]);
    let mut a = &inverse * &z;

    a = a * (1. / (&z.transpose() * &inverse).dot(&z.transpose()));

    let result = &abundonces - &a * (z.dot(&abundonces) - 1.);
    result.as_slice().to_vec()
}
