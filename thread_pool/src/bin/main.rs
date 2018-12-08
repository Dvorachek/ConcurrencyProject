use std::thread;
use std::time::Duration;

extern crate thread_pool;
use thread_pool::{ThreadPool, Computer};

// Used primarily for testing the custom thread_pool library
fn main() {

    // create CPUs for generating latency in worker threads
    let cpu1 = Computer {
        mean : 0.0,
        std : 1.0,
        work_time_increase_factor : 1.0
    };

    let cpu2 = Computer {
        mean : 1.0,
        std : 1.0,
        work_time_increase_factor : 0.5
    };

    let cpu3 = Computer {
        mean : 1.0,
        std : 2.0,
        work_time_increase_factor : 2.25
    };

    let cpu4 = Computer {
        mean : 2.0,
        std : 2.0,
        work_time_increase_factor : 3.5
    };

    let cpu5 = Computer {
        mean : 2.0,
        std : 3.0,
        work_time_increase_factor : 5.0
    };

    let cpus : Vec<Computer> = vec![cpu1, cpu2, cpu3, cpu4, cpu5];

    let pool = ThreadPool::new(cpus);

    for _ in 0..5 {
        pool.execute(|| {
            letscount();
        });

        pool.execute(|| {
            println!("{}", add(3.9, 5.4));
        });

        pool.execute(|| {
            wait();
        });
    }
}

fn letscount() {
    for i in 1..10 {
        println!("test {}", i);
    }
}

fn add(a: f64, b: f64) -> f64 {
    let sum = a + b;

    sum
}

fn wait() {
    thread::sleep(Duration::from_secs(4));
    println!("wait done!");
}
