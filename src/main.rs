use std::sync::mpsc;

extern crate rand;
use rand::distributions::{Exp, Distribution};
use rand::Rng;

extern crate thread_pool;
use thread_pool::{ThreadPool, Computer};

extern crate physics;
use physics::{Body, Simulator, WorkDone};

fn main() {

    // INIT SIMULATOR AND THREADPOOL
    let mean_stars = 2.0;
    let mean_planets = 10.0;
    let mean_others = 250.0;
    let bodies = generate_bodies(mean_stars, mean_planets, mean_others);
    let mut sim = Simulator::new(bodies, 0.0, 60.0);
    let pool = ThreadPool::new(computers_init());

    // CHANNELS FOR RETURNING VALUES FROM THREADPOOL
    let (tx, rx) = mpsc::channel();

    // Run simulation for a number of steps
    let number_simulation_steps = 2;
    for _ in 0..number_simulation_steps {
        // Compute work
        for body in &sim.bodies {
            let id = body.id.clone();
            let tx1 = mpsc::Sender::clone(&tx);
            let sim_clone = sim.clone();
            pool.execute(move || {
                let work = sim_clone.do_work(vec![id]);
                tx1.send(work).unwrap();
            });
        }

        // Get computed work
        let mut work_done : Vec<WorkDone> = vec![];
        for _ in 0..sim.bodies.len() {
            work_done.append(&mut rx.recv().unwrap());
        }

        // Step simulation forward in time
        sim.step_forward(&work_done);
        println!("sim time: {}", sim.time);
    }

}

// PROBABLY OVERKILL, BUT COULD MAKE PUT IN -> impl Computer {
fn computers_init() -> Vec<Computer> {

    let cpu1 = Computer {
        mean : 0.0,
        std : 1.0,
        work_time_increase_factor : 1.0
    };

    let cpu2 = Computer {
        mean : 1.0,
        std : 2.0,
        work_time_increase_factor : 0.0
    };

    let cpu3 = Computer {
        mean : 2.0,
        std : 2.0,
        work_time_increase_factor : 3.0
    };

    let cpu4 = Computer {
        mean : 2.0,
        std : 3.0,
        work_time_increase_factor : 1.5
    };

    let cpus : Vec<Computer> = vec![cpu1, cpu2, cpu3, cpu4];

    cpus
}

fn bodies_init() -> Vec<Body> {

    let earth = Body {
        id : 0,
        position : [149597900000.0, 0.0, 0.0],
        velocity : [0.0, 29800.0, 0.0],
        mass : 5.972e24
    };

    let sun = Body {
        id : 1,
        position : [0.0, 0.0, 0.0],
        velocity : [0.0, 0.0, 0.0],
        mass : 1.989e30
    };

    let bodies : Vec<Body> = vec![earth, sun];

    bodies
}

fn generate_bodies(mean_stars: f64, mean_planets: f64, mean_others: f64) -> Vec<Body> {
    let mut bodies : Vec<Body> = Vec::new();
    let stellar_mass : f64 = 1.989e30;
    let au : f64 = 1.49597e11;
    let mut rng = rand::thread_rng();

    // Generate stars
    let mut exp = Exp::new(mean_stars.powf(-1.0));
    let number_of_stars = exp.sample(&mut rand::thread_rng()).ceil() as usize;

    let mut velocity_scale = 3000.0;
    for _ in 0..number_of_stars {
        let x : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let y : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let z : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;

        let vx : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vy : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vz : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;

        let body = Body {
            id : 0,
            position : [x, y, z],
            velocity : [vx, vy, vz],
            mass : 5.0 * exp.sample(&mut rand::thread_rng()) * stellar_mass
        };
        bodies.push(body);
    }

    // Generate planets
    exp = Exp::new(mean_planets.powf(-1.0));
    let earth_mass : f64 = 5.972e24;
    let number_of_planets = exp.sample(&mut rand::thread_rng()).ceil() as usize;
    velocity_scale = 30000.0;
    for _ in 0..number_of_planets {
        let x : f64 = 4.0 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let y : f64 = 4.0 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let z : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;

        let vx : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vy : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vz : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;

        let body = Body {
            id : 0,
            position : [x, y, z],
            velocity : [vx, vy, vz],
            mass : 50.0 * exp.sample(&mut rand::thread_rng()) * earth_mass
        };
        bodies.push(body);
    }

    // Generate small bodies
    exp = Exp::new(mean_others.powf(-1.0));
    let other_mass : f64 = 2.0e15;
    let number_of_other = exp.sample(&mut rand::thread_rng()).ceil() as usize;
    velocity_scale = 300.0;
    for _ in 0..number_of_other {
        let x : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let y : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let z : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;

        let vx : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vy : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vz : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;

        let body = Body {
            id : 0,
            position : [x, y, z],
            velocity : [vx, vy, vz],
            mass : 5.0e5 * exp.sample(&mut rand::thread_rng()) * other_mass
        };
        bodies.push(body);
    }

    // Ensure high mass bodies are near the end
    bodies.reverse();
    let mut id : usize = 0;
    for body in &mut bodies {
        body.id = id.clone();
        id += 1;
    }
    bodies
}
