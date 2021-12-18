use crate::csv_processing::{sig_samples, Dataframe};
use rayon::{prelude::*};
extern crate nalgebra as na;
use indicatif::ParallelProgressIterator;
use na::{DMatrix, DVector};

pub fn unmix(pixel: &Vec<f64>, signature: &Vec<Vec<f64>>) -> Vec<f64> {

 
    let nsigs = signature.len();
    let nbnds = signature[0].len();

    let pixel = DVector::from_vec(pixel.clone());
    let sig = DMatrix::from_vec(nsigs,nbnds,ravel(signature));
    let sig = sig.transpose();

    //let least_square = sig.lu().solve(&pixel).expect("Linear resolution failed.");;
    
    
    let ls = sig.transpose() * &sig;

    let ls = ls.try_inverse().unwrap();
    let ls = ls * &sig.transpose();
    let res = ls * pixel;
    
    let umixed_vec:Vec<f64> = res.as_slice().to_vec();

    return umixed_vec;



}  

pub fn unmix_all(raster: Vec<Vec<f64>>, signatures: Dataframe) -> Vec<Vec<f64>> {
    let signature_samples = sig_samples(signatures, 0);
    let raster: Vec<Vec<f64>> = transpose(raster);

    let unmixed = raster
        .par_iter()
        .progress()
        .map(|pixel| {
            let unmixed_pixel_results: Vec<Vec<f64>> = signature_samples
                .iter()
                .map(|sig| unmix(pixel, sig))
                .collect();

            let n_samples = unmixed_pixel_results.len() as f64;
            sum_columnwise(unmixed_pixel_results)
                .iter()
                .map(|sum| sum / n_samples)
                .collect()
        })
        .collect();

        transpose(unmixed)
}

fn sum_columnwise(vec: Vec<Vec<f64>>) -> Vec<f64> {
    let ncols = vec[0].len();
    let mut res = vec![0.; ncols];
    for pxl in vec.iter() {
        for i in 0..ncols {
            res[i] += pxl[i]
        }
    }
    res
}
pub fn transpose(vec: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let ncols = vec[0].len();
    (0..ncols)
        .into_par_iter()
        .map(|i| {
            vec.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<f64>>()
        })
        .collect()
}

pub fn ravel(vector:&Vec<Vec<f64>>) -> Vec<f64> {
    let mut result:Vec<f64> = vec![];
    let vector = vector.clone();
    for i in vector.into_iter() {
        for j in i.into_iter() {
            result.push(
                j
            );
        }
    }
    result
}