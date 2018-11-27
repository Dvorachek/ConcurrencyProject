use std::thread;
use std::time::Duration;

extern crate thread_pool;
use thread_pool::ThreadPool;

extern crate physics;
use physics::Body;
use physics::Simulator;


fn main() {

    // TESTING PHYSICS IMPORTS
    let earth = Body {
        position : [149597900000.0, 0.0, 0.0],
        velocity : [0.0, 29800.0, 0.0],
        mass : 5.972e24
    };

    let sun = Body {
        position : [0.0, 0.0, 0.0],
        velocity : [0.0, 0.0, 0.0],
        mass : 1.989e30
    };

    let bodies : Vec<Body> = vec![sun, earth];

    let mut sim = Simulator {
        bodies : bodies,
        time : 0.0,
        time_step : 60.0
    };

    // TESTING THREADPOOL IMPORTS
    //let pool = ThreadPool::new(5);  // this is commented to avoid panick without loop

    // TODO: ADD EXAMPLE OF SIMULATOR USEAGE
    // - made Simulator, Body and WorkDone structs pub along with attributes
    //   might have to do same to Simulator functions
    //   see thread_pool/src/lib.rs for example

    // TODO: RUN SIMULATOR WITH THREADPOOL
}

