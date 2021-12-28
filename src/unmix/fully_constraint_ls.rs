use super::util::{add, ravel, substract, times};
use nalgebra::{DMatrix, DVector};
pub fn unmix_fully_constraint(pixel: &Vec<f64>, signatures: &Vec<Vec<f64>>) -> Vec<f64> {
    let n_em = signatures.len();
    let f = |param: &Vec<f64>| -> f64 {
        let nrows = signatures.len();
        let ncols = signatures[0].len();

        let param_vec = DVector::from_vec(param.clone());
        let pixel_vec = DVector::from_vec(pixel.clone());
        let signatures = DMatrix::from_row_slice(nrows, ncols, ravel(signatures).as_slice());
        let signatures = signatures.transpose();


        let res = &signatures * &param_vec;
        let res = (&res - &pixel_vec).norm();

      
        let con = (1. - param_vec.sum()).powf(2.) * res;
        let bounds = if param.iter().any(|x| x < &0. || x > &1.) {
            999999999.
        } else {
            0.
        };

        res + con + bounds
    };

    let unmixed = nelder_mead(
        &f,
        &mut vec![0.5; n_em],
        0.45,
        0.0001,
        10,
        100,
        1.,
        2.,
        0.5,
        0.5,
    )
    .x;

    //let unmixed = particle_swarm(pixel, signatures);

    unmixed.iter().map(|x| (x * 100.).round() / 100.).collect()
}

#[derive(Clone, Debug)]
pub struct NMResult {
    pub x: Vec<f64>,
    pub score: f64,
}

pub fn nelder_mead(
    f: &dyn Fn(&Vec<f64>) -> f64,
    x_start: &mut Vec<f64>,
    step: f64,
    no_improve_thr: f64,
    no_improve_break: usize,
    max_iter: usize,
    alpha: f64,
    gamma: f64,
    rho: f64,
    sigma: f64,
) -> NMResult {
    let dim = x_start.len();
    let mut prev_best = f(&x_start);
    let mut no_improv = 0;

    let mut res = vec![NMResult {
        x: x_start.clone(),
        score: prev_best,
    }];

    for i in 0..dim {
        let mut x = x_start.clone();
        x[i] += step;
        let score = f(&x);
        res.push(NMResult {
            x: x.clone(),
            score: score,
        });
    }
    let mut iters = 0;

    loop {
        // order
        res.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        let best = res[0].score;

        // break after max_iter
        if max_iter & iters >= max_iter {
            return res[0].clone();
        }
        iters += 1;

        // break after no_improv_break iterations with no improvement
        if best < prev_best - no_improve_thr {
            no_improv = 0;
            prev_best = best;
        } else {
            no_improv += 1;
        }
        if no_improv >= no_improve_break {
            return res[0].clone();
        }

        // centroid
        let mut x0 = vec![0.0; dim];
        for r in res[0..res.len() - 1].iter() {
            for i in 0..r.x.len() {
                x0[i] += r.x[i] / (res.len() - 1) as f64
            }
        }

        // reflection
        let xr = add(&x0, &times(alpha, &substract(&x0, &res[res.len() - 1].x)));
        let rscore = f(&xr);
        if res[0].score <= rscore {
            if rscore < res[res.len() - 2].score {
                res.remove(res.len() - 1);
                res.push(NMResult {
                    x: xr.clone(),
                    score: rscore,
                });
                continue;
            }
        }

        // expansion
        if rscore < res[0].score {
            let xe = add(&x0, &times(gamma, &substract(&x0, &res[res.len() - 1].x)));
            let escore = f(&xe);
            if escore < rscore {
                res.remove(res.len() - 1);
                res.push(NMResult {
                    x: xe.clone(),
                    score: escore,
                });
                continue;
            } else {
                res.remove(res.len() - 1);
                res.push(NMResult {
                    x: xr.clone(),
                    score: rscore,
                });
                continue;
            }
        }

        // contraction
        let xc = add(&x0, &times(rho, &substract(&x0, &res[res.len() - 1].x)));
        let cscore = f(&xc);
        if cscore < res[res.len() - 1].score {
            res.remove(res.len() - 1);
            res.push(NMResult {
                x: xc,
                score: cscore,
            });
            continue;
        }

        //reduction
        let x1 = res[0].x.clone();
        let mut nres = vec![];
        for r in res.iter() {
            let redx = add(&x1, &times(sigma, &substract(&r.x, &x1)));
            let score = f(&redx);
            nres.push(NMResult {
                x: redx,
                score: score,
            })
        }
        res = nres;
    }
}
