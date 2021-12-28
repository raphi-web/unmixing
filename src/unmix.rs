mod constraint_ls;
mod fully_constraint_ls;
mod unconstraint_ls;
mod util;

extern crate nalgebra as na;

use crate::csv_processing::{sig_samples, Dataframe};
use constraint_ls::unmixing_constraint;
use fully_constraint_ls::unmix_fully_constraint;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use unconstraint_ls::unmix;
use util::{sum_columnwise, transpose};

pub fn unmix_all(raster: Vec<Vec<f64>>, signatures: Dataframe, model: String) -> Vec<Vec<f64>> {
    let signature_samples = sig_samples(signatures, 0);
    let raster: Vec<Vec<f64>> = transpose(raster);

    let mix_model_function = match model.as_str() {
        "clsu" => unmixing_constraint,
        "ulsu" => unmix,
        "flsu" => unmix_fully_constraint,
        _ => panic!("\nChoose one of these Models:\n'clsu'\n'ulsu'\n'flsu'\n\n"),
    };

    let unmixed = raster
        .par_iter()
        .progress()
        .map(|pixel| {
            let unmixed_pixel_results: Vec<Vec<f64>> = signature_samples
                .iter()
                .map(|sig| mix_model_function(pixel, sig))
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
