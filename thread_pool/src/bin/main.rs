use std::thread;
use std::time::Duration;

extern crate thread_pool;
use thread_pool::ThreadPool;

// Used primarily for testing the custom thread_pool library
fn main() {
    let pool = ThreadPool::new(5);

    loop{
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
    let mut sum = a + b;

    sum
}

fn wait() {
    thread::sleep(Duration::from_secs(4));
    println!("wait done!");
}
