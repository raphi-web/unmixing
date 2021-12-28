use csv;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Dataframe {
    pub shape: (usize, usize),
    pub header: Option<Vec<String>>,
    pub values: Vec<Vec<f64>>,
}

impl Dataframe {
    pub fn new(filepath: &Path, header: bool, deliminator: u8) -> Self {
        let mut record_vec: Vec<Vec<f64>> = Vec::new();

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(header)
            .delimiter(deliminator)
            .from_path(filepath)
            .expect("Error reading CSV-File!");

        let csv_header = if header {
            let mut headers: Vec<String> = Vec::new();
            for h in rdr.headers().unwrap() {
                headers.push(h.into());
            }

            Some(headers)
        } else {
            None
        };

        for result in rdr.deserialize().into_iter() {
            let record: Vec<f64> = result.unwrap();
            record_vec.push(record);
        }
        let csv_shape = (record_vec.len(), record_vec[0].len());

        Self {
            shape: csv_shape,
            header: csv_header,
            values: record_vec,
        }
    }

    pub fn column(&self, n: usize) -> Vec<f64> {
        let mut col_values: Vec<f64> = vec![];
        for row in self.values.iter() {
            col_values.push(row[n].clone());
        }
        col_values
    }

    pub fn unique(&self, index: usize, axis: usize) -> Vec<f64> {
        let vector: Vec<f64> = if axis == 0 {
            self.values[index].clone()
        } else {
            self.column(index)
        };

        let mut unique_values: Vec<f64> = Vec::new();
        for val in vector.iter() {
            let mut is_unique = true;
            for uval in unique_values.iter() {
                if val == uval {
                    is_unique = false;
                }
            }
            if is_unique {
                unique_values.push(val.clone())
            }
        }

        return unique_values;
    }
    pub fn split_by(&self, col_idx: usize) -> Vec<Dataframe> {
        let unique_values = self.unique(col_idx, 1);
        let mut splitted: Vec<Dataframe> = Vec::new();
        for uval in unique_values.iter() {
            let mut val_vec: Vec<Vec<f64>> = vec![];
            for row in self.values.iter() {
                if row[col_idx] == *uval {
                    let mut r = row.clone();
                    r.remove(col_idx);
                    val_vec.push(r);
                }
            }
            let df = Dataframe {
                shape: (val_vec.len(), val_vec[0].len()),
                header: self.header.clone(),
                values: val_vec,
            };
            splitted.push(df);
        }
        splitted
    }
}
pub fn draw_first(signatures: &mut Vec<Dataframe>) -> Vec<Vec<f64>> {
    
    let mut sigs = vec![];
    for df in signatures.iter_mut() {
      
        let sig = df.values.remove(0);
        sigs.push(sig);
    }
    sigs
}

pub fn sig_samples(df: Dataframe, class_id: usize) -> Vec<Vec<Vec<f64>>> {
    let mut by_class = df.split_by(class_id);
    let minimum = by_class.iter().map(|cdf| cdf.values.len()).min().unwrap();

    (0..minimum).map(|_| draw_first(&mut by_class)).collect()
}
