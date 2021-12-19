pub fn ravel(vector:&Vec<Vec<f64>>) -> Vec<f64> {
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