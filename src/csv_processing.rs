use csv;
use std::path::Path;
use rand::Rng;

#[derive(Debug)]
pub struct Dataframe {
    pub shape: (usize, usize),
    pub header: Option<Vec<String>>,
    pub values: Vec<Vec<String>>,
}

impl Dataframe {
    pub fn new(filepath: &Path, header: bool, deliminator: u8) -> Self {
        let mut record_vec: Vec<Vec<String>> = Vec::new();

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
            let record: Vec<String> = result.unwrap();
            record_vec.push(record);
        }
        let csv_shape = (record_vec.len(), record_vec[0].len());

        Self {
            shape: csv_shape,
            header: csv_header,
            values: record_vec,
        }
    }

    pub fn column(&self, n: usize) -> Vec<String> {
        let mut col_values: Vec<String> = vec![];
        for row in self.values.iter() {
            col_values.push(row[n].clone());
        }
        col_values
    }

    pub fn unique(&self, index: usize, axis: usize) -> Vec<String> {
        let vector: Vec<String> = if axis == 0 {
            self.values[index].clone()
        } else {
            self.column(index)
        };

        let mut unique_values: Vec<String> = Vec::new();
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
            let mut val_vec: Vec<Vec<String>> = vec![];
            for row in self.values.iter() {
                if row[col_idx] == *uval {
                        let mut  r = row.clone();
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

pub fn draw_random(signatures:  &mut Vec<Dataframe>) ->Vec<Vec<String>>{     
    let mut sigs = vec![];
    
    for df in signatures.iter_mut() {
        let shape = df.shape;
        let rand_num:usize = rand::thread_rng().gen_range(0..shape.0);

        let sig = df.values.remove(rand_num);
        sigs.push(sig);
    }
    sigs
}
