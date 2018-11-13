const GRAVITATIONAL_CONSTANT : f64 = 6.67408e-11;

struct Force {
    force : [f64 ; 3]
}

struct Body {
    position : [f64 ; 3],
    velocity : [f64 ; 3],
    mass : f64
}

struct Simulator {
    bodies : Vec<Body>,
    time_step : f64,
    time : f64
}

fn vector_scalar_multiple(vector : &[f64 ; 3], scalar : &f64) -> [f64 ; 3] {
    let mut multiple : [f64 ; 3] = [0.0, 0.0, 0.0];
    for index in 0..3 {
        multiple[index] = scalar * vector[index];
    }

    multiple
}

fn vector_sum(a : &[f64 ; 3], b : &[f64; 3]) -> [f64 ; 3] {
    let mut sum : [f64 ; 3] = [0.0, 0.0, 0.0];
    for index in 0..3 {
        sum[index] = a[index] + b[index];
    }

    sum
}

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
