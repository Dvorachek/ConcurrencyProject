use std::sync::mpsc;

extern crate rand;
use rand::distributions::{Exp, Distribution};
use rand::Rng;

extern crate thread_pool;
use thread_pool::{ThreadPool, Computer};

extern crate physics;
use physics::{Body, Simulator, WorkDone};

extern crate piston_window;
extern crate find_folder;
use piston_window::*;

const AU: f64 = 149.6e9; // Astronomical Unit in meters, roughly distance earth -> sun
const SCALE: f64 = 50.0 / AU;
const DIMENSION: u32 = 1000;
const HALF: f64 = AU * 10.0;

fn main() {

    // INIT SIMULATOR AND THREADPOOL
    let mean_stars = 2.0;
    let mean_planets = 10.0;
    let mean_others = 20.0;
    let bodies = generate_bodies(mean_stars, mean_planets, mean_others);
    let mut sim = Simulator::new(bodies, 0.0, 10000.0);
    let pool = ThreadPool::new(computers_init());

    // CHANNELS FOR RETURNING VALUES FROM THREADPOOL
    let (tx, rx) = mpsc::channel();

    // INIT PISTONWINDOW
    let mut window: PistonWindow = 
        WindowSettings::new(
            "piston: draw_state",
            Size {
                width: DIMENSION,
                height: DIMENSION,
            },
        ).exit_on_esc(true).build().unwrap();

    // piston window lazy means that only events will tricker a step
    window.set_lazy(false);

    // LOOP DRAWING
    while let Some(e) = window.next() {

        let chunk_size = 2;
        let mut bodies = sim.bodies.clone();

        // DRAW HERE
        window.draw_2d(&e, |c, g| {
            clear([0.129, 0.1468, 0.168, 1.0], g); // ?????
            g.clear_stencil(0);
            
            // Compute work
            let mut ids : Vec<usize> = Vec::new();
            for chunk in bodies.chunks(chunk_size) {
                for body in chunk {
                    ids.push(body.id.clone());
                }
                let tx1 = mpsc::Sender::clone(&tx);
                let sim_clone = sim.clone();
                let ids_clone = ids.clone();
                pool.execute(move || {
                    let work = sim_clone.do_work(ids_clone);
                    tx1.send(work).unwrap();
                });
                ids = Vec::new();
            }

            // Get computed work
            let mut work_done : Vec<WorkDone> = vec![];
            while work_done.len() < sim.bodies.len() {
                work_done.append(&mut rx.recv().unwrap());
            }

            // Step simulation forward in time
            sim.step_forward(&work_done);
            println!("sim time: {}", sim.time);

            for body in &sim.bodies {
                draw_body(&body, c, g);
            }
        });
    }
}

fn draw_body(body: &Body, c: Context, g: &mut G2d) {
    Ellipse::new(body.colour) // colour
        .draw(
            [body.position[0] * SCALE + 500.0, body.position[1] * SCALE + 500.0,
                body.radius, body.radius],  // radius radius
            &c.draw_state, c.transform, g
        );
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
            mass : 5.0 * exp.sample(&mut rand::thread_rng()) * stellar_mass,
            colour : [255.0, 255.0, 0.0, 1.0],
            radius : 15.0,
        };
        bodies.push(body);
    }

    // Generate planets
    exp = Exp::new(mean_planets.powf(-1.0));
    let earth_mass : f64 = 5.972e24;
    let number_of_planets = exp.sample(&mut rand::thread_rng()).ceil() as usize;
    velocity_scale = 30000.0;
    for _ in 0..number_of_planets {
        let x : f64 = 8.0 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let y : f64 = 8.0 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let z : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;

        let vx : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vy : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vz : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;

        let body = Body {
            id : 0,
            position : [x, y, z],
            velocity : [vx, vy, vz],
            mass : 50.0 * exp.sample(&mut rand::thread_rng()) * earth_mass,
            colour : [0.69803, 0.186215, 0.12549, 1.0],
            radius : 10.0
        };
        bodies.push(body);
    }

    // Generate small bodies
    exp = Exp::new(mean_others.powf(-1.0));
    let other_mass : f64 = 2.0e15;
    let number_of_other = exp.sample(&mut rand::thread_rng()).ceil() as usize;
    velocity_scale = 300.0;
    for _ in 0..number_of_other {
        let x : f64 = 16.0 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let y : f64 = 16.0 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;
        let z : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * au;

        let vx : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vy : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vz : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;

        let body = Body {
            id : 0,
            position : [x, y, z],
            velocity : [vx, vy, vz],
            mass : 5.0e5 * exp.sample(&mut rand::thread_rng()) * other_mass,
            colour : [0.298039, 0.7705882, 0.411765, 1.0],
            radius : 8.0,
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
