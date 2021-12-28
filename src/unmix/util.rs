use rayon::prelude::*;

pub fn ravel(vector: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    let vector = vector.clone();
    for i in vector.into_iter() {
        for j in i.into_iter() {
            result.push(j);
        }
    }
    result
}

pub fn sum_columnwise(vec: Vec<Vec<f64>>) -> Vec<f64> {
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

pub fn substract(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![];
    for i in 0..a.len() {
        let x = a[i] - b[i];
        res.push(x);
    }

    res
}

pub fn add(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![];
    for i in 0..a.len() {
        res.push(a[i] + b[i])
    }
    res
}

pub fn times(con: f64, vector: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![];
    for i in 0..vector.len() {
        res.push(vector[i] * con)
    }
    res
}
