const GRAVITATIONAL_CONSTANT : f64 = 6.67408e-11;

#[derive(Clone)]
pub struct Body {
    pub id : usize,
    pub position : [f64 ; 3],
    pub velocity : [f64 ; 3],
    pub mass : f64
}

#[derive(Clone)]
pub struct Simulator {
    pub bodies : Vec<Body>,
    pub time_step : f64,
    pub time : f64
}

pub struct WorkDone {
    force : [f64 ; 3],
    body_index : usize
}

impl Simulator {
    pub fn new(bodies : Vec<Body>, time : f64, time_step : f64) -> Simulator {
        Simulator {
            bodies : bodies,
            time : time,
            time_step : time_step
        }
    }

    fn compute_force(&self, origin : &Body, attractor : &Body) -> [f64; 3] {
        let distance : f64 = vector_magnitude(&vector_difference(&attractor.position, &origin.position));
        let force_magnitude = GRAVITATIONAL_CONSTANT * origin.mass * attractor.mass / distance.powi(2);
        let rhat = normalize(&vector_difference(&origin.position, &attractor.position));

        vector_scalar_multiple(&rhat, &force_magnitude)
    }

    pub fn step_forward(&mut self, work : &Vec<WorkDone>) {
        for work_done in work {
            let body = &mut self.bodies[work_done.body_index];
            euler_cromer_integrate(body, &work_done.force, &self.time_step);
        }
        self.time += self.time_step;
    }

    pub fn do_work(&self, ids : Vec<usize>) -> Vec<WorkDone> {
        let mut work_done : Vec<WorkDone> = Vec::new();

        for id in ids {
            let origin : &Body = &self.bodies[id];
            let mut force : [f64 ; 3] = [0.0, 0.0, 0.0];

            for attractor in &self.bodies {
                // A body does not attract itself
                if id == attractor.id {
                    continue;
                }

                let f : [f64 ; 3] = self.compute_force(origin, attractor);
                force = vector_sum(&force, &f);
            }

            work_done.push(WorkDone {
                force : force,
                body_index : id
            });
        }
        work_done
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
