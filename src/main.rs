use std::sync::mpsc;

extern crate thread_pool;
use thread_pool::{ThreadPool, Computer};

extern crate physics;
use physics::{Body, Simulator, WorkDone};

fn main() {

    // INIT SIMULATOR AND THREADPOOL
    let mut sim = Simulator::new(bodies_init(), 0.0, 60.0);
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
