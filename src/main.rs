use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

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

const AU: f64 = 1.49597e11;
const SCALE: f64 = 50.0 / AU;
const DIMENSION: u32 = 1000;
const DAY: f64 = 24.0 * 3600.0;

fn main() {
    // INIT SIMULATOR AND THREADPOOL
    let mean_stars = 2.0;
    let mean_planets = 10.0;
    let mean_others = 20.0;
    let bodies = generate_bodies(mean_stars, mean_planets, mean_others);
    let sim = Simulator::new(bodies, 0.0, DAY);
    let thread_pool = ThreadPool::new(computers_init());

    let chunk_size = sim.bodies.len() / 4;

    render(chunk_size, sim, thread_pool);
}

fn render(chunk_size: usize, mut sim: Simulator, thread_pool: ThreadPool) {
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

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font = assets.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    // piston window lazy means that only events will tricker a step
    window.set_lazy(false);

    // LOOP DRAWING
    while let Some(e) = window.next() {
        // DRAW HERE
        window.draw_2d(&e, |c, g| {
            clear([0.129, 0.1468, 0.168, 1.0], g);
            g.clear_stencil(0);

            let transform = c.transform.trans(820.0, 980.0);  // position of text X Y
            text::Text::new_color([255.0, 255.0, 255.0, 1.0], 32).draw(
                &format!("Day {}", &sim.time / DAY),
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            let work_done = distribute_work(&thread_pool, &sim, chunk_size.clone(), &tx, &rx);

            // Step simulation forward in time
            sim.step_forward(&work_done);

            for body in &sim.bodies {
                draw_body(&body, c, g);
            }
        });
    }
}

fn distribute_work(threadpool: &ThreadPool,
                   sim: &Simulator,
                   chunk_size: usize,
                   tx: &Sender<Vec<WorkDone>>,
                   rx: &Receiver<Vec<WorkDone>>) -> Vec<WorkDone> {
    // Distribute work
    let mut ids : Vec<usize> = Vec::new();
    for chunk in sim.bodies.chunks(chunk_size) {
        for body in chunk {
            ids.push(body.id.clone());
        }

        let tx1 = mpsc::Sender::clone(tx);
        let sim_clone = sim.clone();
        threadpool.execute(move || {
            let work = sim_clone.do_work(ids.clone());
            tx1.send(work).unwrap();
        });

        ids = Vec::new();
    }

    // Get computed work
    let mut work_done : Vec<WorkDone> = vec![];
    while work_done.len() < sim.bodies.len() {
        work_done.append(&mut rx.recv().unwrap());
    }

    work_done
}

fn draw_body(body: &Body, c: Context, g: &mut G2d) {
    Ellipse::new(body.colour)
        .draw(
            [body.position[0] * SCALE + 500.0, body.position[1] * SCALE + 500.0,  // X, Y
                body.radius, body.radius],
            &c.draw_state, c.transform, g
        );
}

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

    // ADJUST PARAM HERE
    let stellar_mass : f64 = 1.989e30 * 5.0;
    let stellar_v_scale : f64 = 3000.0;
    let stellar_dist_scale : f64 = 1.0;
    let stellar_colour : [f32; 4] = [255.0, 255.0, 0.0, 1.0];
    let stellar_radius : f64 = 15.0;

    let earth_mass : f64 = 5.972e24 * 50.0;
    let planet_v_scale : f64 = 30000.0;
    let planet_dist_scale : f64 = 8.0;
    let planet_colour : [f32; 4] = [0.69803, 0.186215, 0.12549, 1.0];
    let planet_radius : f64 = 10.0;

    let other_mass : f64 = 2.0e15 * 5.0e5;
    let other_v_scale : f64 = 300.0;
    let other_dist_scale : f64 = 16.0;
    let other_colour : [f32; 4] = [0.298039, 0.7705882, 0.411765, 1.0];
    let other_radius : f64 = 8.0;


    bodies.append(&mut make_body_vec(stellar_mass,
        mean_stars,
        stellar_v_scale,
        stellar_dist_scale,
        stellar_colour,
        stellar_radius));
    bodies.append(&mut make_body_vec(earth_mass,
        mean_planets,
        planet_v_scale,
        planet_dist_scale,
        planet_colour,
        planet_radius));
    bodies.append(&mut make_body_vec(other_mass,
        mean_others,
        other_v_scale,
        other_dist_scale,
        other_colour,
        other_radius));

    // Ensure high mass bodies are near the end
    bodies.reverse();
    let mut id : usize = 0;
    for body in &mut bodies {
        body.id = id.clone();
        id += 1;
    }
    bodies
}

fn make_body_vec(mass: f64, mean: f64, velocity_scale: f64, distance_scale: f64, colour: [f32; 4], radius: f64) -> Vec<Body> {
    let mut bodies : Vec<Body> = Vec::new();
    let mut rng = rand::thread_rng();

    let exp = Exp::new(mean.powf(-1.0));
    let number_of_bodies = exp.sample(&mut rand::thread_rng()).ceil() as usize;

    for _ in 0..number_of_bodies {
        let x : f64 = distance_scale * (-1.0 + 2.0 * rng.gen::<f64>()) * AU;
        let y : f64 = distance_scale * (-1.0 + 2.0 * rng.gen::<f64>()) * AU;
        let z : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * AU;

        let vx : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vy : f64 = (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;
        let vz : f64 = 0.01 * (-1.0 + 2.0 * rng.gen::<f64>()) * velocity_scale;

        let body = Body {
            id : 0,
            position : [x, y, z],
            velocity : [vx, vy, vz],
            mass : exp.sample(&mut rand::thread_rng()) * mass,
            colour : colour,
            radius : radius,
        };
        bodies.push(body);
    }

    bodies
}
