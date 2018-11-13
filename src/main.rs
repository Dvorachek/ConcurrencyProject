const GRAVITATIONAL_CONSTANT : f64 = 6.67408e-11;

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

struct WorkDone {
    force : [f64 ; 3],
    body_index : usize
}

impl Simulator {
    fn compute_force(&self, origin : &Body, attractor : &Body) -> [f64; 3] {
        let distance : f64 = vector_magnitude(&vector_difference(&attractor.position, &origin.position));
        let force_magnitude = GRAVITATIONAL_CONSTANT * origin.mass * attractor.mass / distance.powi(2);
        let rhat = normalize(&vector_difference(&origin.position, &attractor.position));

        vector_scalar_multiple(&rhat, &force_magnitude)
    }

    fn step_forward(&mut self, work : &mut Vec<WorkDone>) {
        for i in 0..work.len() {
            let index = work[i].body_index;
            let body = &mut self.bodies[index];
            euler_cromer_integrate(body, &work[i].force, &self.time_step);
        }
        self.time += self.time_step;
    }
}

fn euler_cromer_integrate(body : &mut Body, force : &[f64 ; 3], time_step : &f64) {
    let acceleration : [f64 ; 3] = vector_scalar_multiple(&force, &body.mass.powi(-1));
    body.velocity = vector_sum(&body.velocity, &vector_scalar_multiple(&acceleration, &time_step));
    body.position = vector_sum(&body.position, &vector_scalar_multiple(&body.velocity, &time_step));
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
