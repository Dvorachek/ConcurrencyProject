use std::thread;
use std::time::Duration;
use std::sync::mpsc;

extern crate thread_pool;
use thread_pool::ThreadPool;

extern crate physics;
use physics::Body;
use physics::Simulator;


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

    let bodies : Vec<Body> = vec![sun, earth];

    let mut sim = Simulator::new(bodies, 0.0, 60.0);

    let pool = ThreadPool::new(5);

    // CHANNELS FOR RETURNING VALUES FROM THREADPOOL
    let (tx, rx) = mpsc::channel();

    // TODO: RUN SIMULATOR WITH THREADPOOL
    for _ in 0..24 {
        let tx1 = mpsc::Sender::clone(&tx);
        let mut cl = sim.clone();

        pool.execute(move || {
            let mut val = cl.do_work();
            tx1.send(val).unwrap();
        });
    }

    for _ in 0..24 {
        let mut received = rx.recv().unwrap();
        sim.step_forward(&mut received);
        println!("sim time: {}", sim.time);
    }

    // TODO: ADD DECONSTRUCTOR FOR THREADPOOL
    loop{
        /*
        pool.execute(|| {
            wait();
        });
        */
    }
}

fn wait() {
    println!("testing wait");
    thread::sleep(Duration::from_secs(4));
    println!("wait done!");
}

