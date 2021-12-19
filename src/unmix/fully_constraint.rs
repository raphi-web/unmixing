
use argmin::solver::linesearch::{MoreThuenteLineSearch};
use argmin::{prelude::*, solver::quasinewton::LBFGS, };

use finitediff::*;
use nalgebra::{DMatrix, DVector};



fn unmixing_function(param: &Vec<f64>, pixel: &Vec<f64>, signatures: &Vec<Vec<f64>>) -> f64 {
    let param = DVector::from_vec(param.clone());
    let pixel = DVector::from_vec(pixel.clone());
    let nrows = signatures.len();
    let ncols = signatures[0].len();
    let signatures = DMatrix::from_row_slice(nrows, ncols, ravel(&signatures).as_slice()).transpose();

    let result = ((&signatures * &param) - &pixel).norm();
    let con = (1. - &param.sum()).abs() * result;
    result + con
}

#[derive(Debug,Clone)]
struct UnmixBlock {
    pixel: Vec<f64>,
    signatures: Vec<Vec<f64>>,
}

impl ArgminOp for UnmixBlock {
    type Param = Vec<f64>;
    type Output = f64;
    type Hessian = ();
    type Jacobian = ();
    type Float = f64;

    fn apply(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(unmixing_function(p, &self.pixel, &self.signatures))
    }

    fn gradient(&self, p: &Self::Param) -> Result<Self::Param, Error> {
        Ok((*p).central_diff(&|x| unmixing_function(&x.to_vec(), &self.pixel, &self.signatures)))
    }
}

pub fn unmix_con(pixel:&Vec<f64>, signatures: &Vec<Vec<f64>>) -> Vec<f64> {
   
    let n_em = signatures.len();
    let n_bnds = pixel.len();
    let cost = UnmixBlock {
        pixel: pixel.clone(),
        signatures: signatures.clone(),
    };  
    
    let linesearch = MoreThuenteLineSearch::new();
    let solver = LBFGS::new(linesearch, n_bnds).with_tol_cost(0.00001);

    let mut scores:Vec<(Vec<f64>,f64)> = vec![];

    //let init_param: Vec<f64> = vec![0.5; n_em];   // Define initial parameter vector
    let init_parmas:Vec<Vec<f64>> = vec![vec![0.75; n_em], vec![0.25; n_em], vec![0.5; n_em]];

    for init_param in init_parmas.into_iter(){
        let res = Executor::new(cost.clone(), solver.clone(), init_param)  // Run solver
        .max_iters(100)
        .run()
        .unwrap();
    
        scores.push((res.state.get_best_param(), res.state.get_best_cost()));

    }    
    scores.sort_by(|a,b| {a.1.partial_cmp(&b.1).unwrap()});
    let res =scores.remove(0);
    res.0
    
}
fn ravel(vector:&Vec<Vec<f64>>) -> Vec<f64> {
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

