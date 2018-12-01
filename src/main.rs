use std::thread;
use std::time::Duration;
use std::sync::mpsc;

extern crate thread_pool;
use thread_pool::ThreadPool;

extern crate physics;
use physics::Body;
use physics::Simulator;
use physics::WorkDone;

fn main() {

    // TESTING PHYSICS IMPORTS
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

    let mut sim = Simulator::new(bodies, 0.0, 60.0);

    let pool = ThreadPool::new(2);

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
                let work = sim_clone.do_work(id);
                tx1.send(work).unwrap();
            });
        }

        // Get computed work
        let mut work_done : Vec<WorkDone> = vec![];
        for _ in 0..sim.bodies.len() {
            work_done.push(rx.recv().unwrap());
        }

        // Step simulation forward in time
        sim.step_forward(&work_done);
        println!("sim time: {}", sim.time);
    }

}

