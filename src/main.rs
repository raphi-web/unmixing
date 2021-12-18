use unmix::{unmix_all};

use std::path::PathBuf;
use structopt::StructOpt;

mod csv_processing;
mod unmix;
mod raster_io;

use raster_io::{read_raster,write_raster};

#[derive(StructOpt)]
#[structopt(name = "basic")]
pub struct Cli {
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
    let input_raster = args.raster;
    let out_raster = args.output;

    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();


    let (raster_bands,mut profile) = read_raster(input_raster);

    
    let  df = csv_processing::Dataframe::new(input_csv, true, ",".as_bytes()[0]);
    let unmixed_raster = unmix_all(raster_bands, df);
    
    profile.band_count = unmixed_raster.len() as isize;
    
    write_raster(out_raster, unmixed_raster, profile);

}
