mod constraint_ls;
mod unconstraint_ls;
use crate::csv_processing::{sig_samples, Dataframe};
use rayon::{prelude::*};
extern crate nalgebra as na;
use indicatif::ParallelProgressIterator;

use constraint_ls::unmixing_constraint;
use unconstraint_ls::unmix;



pub fn unmix_all(raster: Vec<Vec<f64>>, signatures: Dataframe, unconstraint:bool) -> Vec<Vec<f64>> {
    let signature_samples = sig_samples(signatures, 0);
    let raster: Vec<Vec<f64>> = transpose(raster);

    let unmixed = raster
        .par_iter()
        .progress()
        .map(|pixel| {
            let unmixed_pixel_results: Vec<Vec<f64>> = signature_samples
                .iter()
                .map(|sig| if unconstraint {unmix(pixel, sig)} else {unmixing_constraint(pixel, sig)})
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