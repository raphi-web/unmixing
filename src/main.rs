use crate::csv_processing::draw_random;
use gdal;
use rayon::prelude::*;
use rayon::vec;
use rayon::ThreadPoolBuilder;
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;
use unmix::{unmix, unmix_raster};
mod csv_processing;
mod unmix;

#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Cli {
    #[structopt(parse(from_os_str))]
    raster: PathBuf,
    #[structopt(parse(from_os_str))]
    csv: PathBuf,
    #[structopt(parse(from_os_str))]
    output: PathBuf,
}
fn main() {
    let args = Cli::from_args();
    let input_csv = args.csv.as_path();
    let input_raster = args.raster.as_path();
    let out_raster = args.raster.as_path();

    let raster_dataset = gdal::Dataset::open(input_raster).expect("Error opening raster file");
    let transform = raster_dataset.geo_transform().unwrap();
    let projection = raster_dataset.projection();

    let band_count = raster_dataset.raster_count();


    rayon::ThreadPoolBuilder::new()
    .num_threads(6)
    .build_global()
    .unwrap();


    let mut raster_bands = vec![];
    for band_number in 1..band_count + 1 {
        let rasterband = raster_dataset
            .rasterband(band_number)
            .expect("Error: Raster-Band could not be read");

        let cols = rasterband.x_size();
        let rows = rasterband.y_size();

        let size: usize = rows * cols;
        let mut rast_vals: Vec<f64> = vec![0.; size as usize];
        let buff = &mut rast_vals[..];

        rasterband
            .read_into_slice((0, 0), (cols, rows), (cols, rows), buff, None)
            .unwrap();

        raster_bands.push(rast_vals)
    }

    let nbands = raster_bands.len();
    let ncols = raster_bands[0].len();

    let rast: Vec<Vec<f64>> = transpose(raster_bands);


    let mut a = csv_processing::Dataframe::new(input_csv, true, ",".as_bytes()[0]).split_by(0);

    let b = draw_random(&mut a);
    let c = [146.21552, 142.08421, 80.00875].to_vec();
    let d: Vec<Vec<f64>> = vec![
        vec![212.6503, 10.482534, 136.97844],
        vec![267.92877, 32.882927, 108.81945],
        vec![181.8901, -5.3892436, 41.599026],
    ];

    let res = unmix(&c, d.clone());
    println!("Single Result:{:?}", res);


    let res = unmix_raster(&rast, d);
}

fn transpose(vec:Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let ncols = vec[0].len();
    (0..ncols)
        .into_par_iter()
        .map(|i| {
            vec
                .iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<f64>>()
        })
        .collect()
    }
