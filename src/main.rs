fn vector_difference(a : &[f64 ; 3], b : &[f64; 3]) -> [f64 ; 3] {
    let mut difference : [f64 ; 3] = [0.0,0.0,0.0];
    for index in 0..3 {
        difference[index] = b[index] - a[index];
    }

    difference
}

fn vector_magnitude(vector : &[f64 ; 3]) -> f64 {
    let mut magnitude : f64 = 0.0;
    for index in 0..3 {
        magnitude += vector[index] * vector[index];
    }

    magnitude.sqrt()
}

fn normalize(vector : &[f64 ; 3]) -> [f64 ; 3] {
    let mut normal_vector : [f64 ; 3] = [0.0, 0.0, 0.0];
    let mag = vector_magnitude(vector);

    for index in 0..3 {
        normal_vector[index] = &vector[index] / mag;
    }

    normal_vector
}

fn main() {

}
