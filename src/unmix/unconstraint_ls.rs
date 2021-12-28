use super::util::ravel;
use nalgebra::{DMatrix, DVector};
pub fn unmix(pixel: &Vec<f64>, signature: &Vec<Vec<f64>>) -> Vec<f64> {
    let nsigs = signature.len();
    let nbnds = signature[0].len();

    let pixel = DVector::from_vec(pixel.clone());
    let sig = DMatrix::from_row_slice(nsigs, nbnds, ravel(signature).as_slice());
    let sig = sig.transpose();

    //let least_square = sig.lu().solve(&pixel).expect("Linear resolution failed.");;

    let ls = sig.transpose() * &sig;
    let ls = ls.pseudo_inverse(1e-12).unwrap();
    let ls = ls * &sig.transpose();
    let res = ls * pixel;

    let umixed_vec: Vec<f64> = res.as_slice().to_vec();

    return umixed_vec;
}
