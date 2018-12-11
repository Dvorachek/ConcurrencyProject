use std::thread;
use std::time::{Duration, Instant};

extern crate thread_pool;
use thread_pool::{ThreadPool, Computer};

// Used primarily for testing the custom thread_pool library
fn main() {

    let pool = ThreadPool::new(computers_init());

    println!("{:?}", pool);

    let start = Instant::now();

    println!("Start: {:?}", start);

    
    for _ in 0..5 {
        
        pool.execute(|| {
            add(3.9, 5.4);
        });

        pool.execute(|| {
            wait();
        });
        
    }

    wait();
    let end = Instant::now().duration_since(start);
    println!("Main thread ran for {:?}", end);
}

fn add(a: f64, b: f64) -> f64 {
    let sum = a + b;

    sum
}

fn wait() {
    thread::sleep(Duration::from_secs(4));
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
