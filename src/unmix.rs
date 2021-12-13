mod nelder_mead;


pub fn unmix(pixel: Vec<f64>, signatures: Vec<Vec<f64>>)  {
 
    let function = |x: &Vec<f64>| {
      
        let mut result: Vec<f64> = vec![];
        for line in signatures.iter() {
            let mut y = 0.;
            for i in 0..line.len() {
                y += line[i] * x[i];
            }
            result.push(y)
        }
        let mut z: Vec<f64> = vec![];
        for i in 0..x.len() {
            z.push((pixel[i] - result[i]) * (pixel[i] - result[i]))
        }

        let z:f64 = z.iter().sum();
        z

    };


    let mut start = vec![0.5,0.5,0.5];
    let res = nelder_mead::nelder_mead(&function, &mut start, 0.1, 0.00005, 10, 100, 1., 2., -0.5, 0.5);
    println!("Result: {:?}",res);
}   
pub fn unmix_raster(rast:Vec<Vec<f64>>, signatures: Vec<Vec<f64>>) {
    let nrows = rast.len();
    let ncols = rast[0].len();

    let mut unmixed_raster:Vec<Vec<f64>> = Vec::new(); 
    for i in 0..ncols {
            let mut pixel:Vec<f64> =vec![];
            for j in 0..nrows {
                let value = rast[i][j];
                pixel.push(value);
            }
            println!("unmix");
            let unmixed_pxl = unmix(pixel, signatures.clone());
            //unmixed_raster.push(unmixed_pxl);

    }

}



