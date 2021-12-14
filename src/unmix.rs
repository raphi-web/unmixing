mod nelder_mead;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use crate::csv_processing::{draw_random, Dataframe};
pub fn unmix(pixel: &Vec<f64>, signatures: Vec<Vec<f64>>) -> Vec<f64> {
    let function = |x: &Vec<f64>| {
        let mut result: Vec<f64> = vec![];
        for line in signatures.iter() {
            let mut y = 0.;
            for i in 0..line.len() {
                // if x[i] smaller 0 make it lgr 0
                // to avoid negative y (results)
                y += line[i] * {
                    if x[i] < 0. {
                        x[i] * -1.
                    } else {
                        x[i]
                    }
                };
            }
            result.push(y)
        }

        let mut z: Vec<f64> = vec![];
        for i in 0..x.len() {
            z.push((pixel[i] - result[i]) * (pixel[i] - result[i]))
        }

        let z: f64 = z.iter().sum();
        z
    };

    let mut start = vec![0.5, 0.5, 0.5];
    let res = nelder_mead::nelder_mead(
        &function, &mut start, 0.1, 0.0005, 10, 100, 1., 2., -0.5, 0.5,
    );
    res.x
}
pub fn unmix_raster(rast: &Vec<Vec<f64>>, signatures: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let ncols = rast[0].len();
    let mut unmixed_raster: Vec<Vec<f64>> = Vec::new();

    let result: Vec<Vec<f64>> = rast
        .par_iter()
        .progress()
        .map(|pxl| unmix(pxl, signatures.clone()))
        .collect();
    result
}

pub fn unmix_all(rast: &Vec<Vec<f64>>, signatures: &mut Vec<Dataframe>) -> Vec<Vec<f64>> {
    let minimum = signatures.iter().map(|s| s.values.len()).min().unwrap();

    // draw signatures the from csv dataframe and convert them to float
    let sig_samples: Vec<Vec<Vec<f64>>> = (0..minimum)
        .map(|_| {
            let sig = draw_random(signatures);
            sig.into_iter()
                .map(|line| {
                    line.into_iter()
                        .map(|v| v.parse::<f64>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // unmix the raster with all the drawn signatures, average the results
    rast.into_par_iter()
        .map(|pxl| {
           
            let mut umix_results: Vec<Vec<f64>> = sig_samples
                .iter()
                .map(|sig| unmix(pxl, sig.clone()))
                .collect();

            let n_unmixed: f64 = umix_results.len() as f64;

            sum_columnwise(umix_results)
                .into_iter()
                .map(|px| px / n_unmixed)
                .collect()
        })
        .collect()
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
