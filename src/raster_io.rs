use std::path::PathBuf;

use gdal::raster::Buffer;

#[derive(Clone, Debug)]
pub struct Profile {
    pub transform: [f64; 6],
    pub projection: String,
    pub band_count: isize,
    pub shape: (usize, usize), // cols , rows
}

pub fn read_raster(fname: PathBuf) -> (Vec<Vec<f64>>, Profile) {
    let raster_dataset = gdal::Dataset::open(fname.as_path()).expect("Error opening raster file");

    let transform = raster_dataset.geo_transform().unwrap();
    let projection = raster_dataset.projection();

    let band_count = raster_dataset.raster_count();
    let mut raster_bands = vec![];
    let mut cols: usize = 0;
    let mut rows: usize = 0;



    for band_number in 1..band_count + 1 {
        let rasterband = raster_dataset
            .rasterband(band_number)
            .expect("Error: Raster-Band could not be read");

        cols = rasterband.x_size();
        rows = rasterband.y_size();

        let size: usize = rows * cols;
        let mut rast_vals: Vec<f64> = vec![0.; size as usize];
        let buff = &mut rast_vals[..];

        rasterband
            .read_into_slice((0, 0), (cols, rows), (cols, rows), buff, None)
            .unwrap();

        raster_bands.push(rast_vals)
    }

    let profile = Profile {
        transform: transform,
        projection: projection,
        band_count: band_count,
        shape: (cols, rows),
    };

    (raster_bands, profile)
}

pub fn write_raster(filename: PathBuf, raster: Vec<Vec<f64>>, profile: Profile) {
    let (cols, rows) = profile.shape;
    let n_bands = profile.band_count;
    let transform = profile.transform;
    let projection = profile.projection;

    let driver = gdal::Driver::get("GTiff").unwrap();
    let mut dataset = driver
        .create_with_band_type::<f64, _>(
            filename.as_path(),
            cols as isize,
            rows as isize,
            n_bands as isize,
        )
        .expect("Could not create output raster");

    dataset
        .set_projection(&projection)
        .expect("Error setting Projection");
    dataset
        .set_geo_transform(&transform)
        .expect("Error setting Geo-Transform");

    for (idx, band) in raster.into_iter().enumerate() {
        let mut rb = dataset.rasterband(idx as isize + 1).unwrap();

        let buff: Buffer<f64> = Buffer {
            size: (cols, rows),
            data: band,
        };

        rb.write((0, 0), (cols, rows), &buff)
            .expect("Error writing new Raster to band");
    }
}
