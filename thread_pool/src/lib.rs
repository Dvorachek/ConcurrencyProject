use std::thread;
use std::time::{Duration, Instant};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

extern crate rand;
use rand::distributions::{Normal, Distribution};


pub struct Computer {
    pub mean : f64,
    pub std : f64,
    pub work_time_increase_factor : f64,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}


#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn new(cpus: Vec<Computer>) -> ThreadPool {
        assert!(cpus.len() > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(cpus.len());

        // Enumerate over cpus and create workers
        for (id, cpu) in cpus.iter().enumerate() {
            workers.push(Worker::new(id, Arc::clone(&receiver), cpu));
        }

        ThreadPool {
            workers,
            sender,
        }
    }

    //pub fn execute_specific_worker() {

    //}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            //println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
    //start_time: f64,

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, cpu: &Computer) ->
        Worker {

        let thread_start = Instant::now();
        let mut work_times : Vec<Duration> = Vec::new();
        let mut latency_sleeps : Vec<f64> = Vec::new();
        

        let normal = Normal::new(cpu.mean, cpu.std);
        let time_scale_factor = cpu.work_time_increase_factor.clone();

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        //println!("Worker {} got a job; executing.", id);

                        let mut latency = normal.sample(&mut rand::thread_rng());
                        // Need latency to be positive
                        while latency < 0.0 {
                            latency = normal.sample(&mut rand::thread_rng());
                        }
                        //for testing
                        println!("Sleeping worker {} with normal distributed latency {}", id, latency);
                        thread::sleep(Duration::from_secs(latency as u64));

                        // Split latency into seconds and milliseconds required by Duration
                        let secs_to_millisecs = 1000.0;
                        let mut secs = latency.floor();
                        let mut millisecs = (latency - secs) * secs_to_millisecs;
                        thread::sleep(Duration::new(secs as u64, millisecs as u32));

                        let start = Instant::now();

                        job.call_box();

                        work_times.push(Instant::now().duration_since(start));
                        latency_sleeps.push(latency);

                        /* Not actually going to work as intended.. Commenting until fixed
                        //let end = Instant::now().duration_since(start);
                        secs = end.as_secs() as f64 * time_scale_factor;
                        millisecs = end.subsec_millis() as f64 * time_scale_factor;
                        let extra_time = Duration::new(secs as u64, millisecs as u32);
                        thread::sleep(extra_time);
                        */
                    },
                    Message::Terminate => {
                        let thread_end = Instant::now().duration_since(thread_start);
                        let mut time_idle = thread_end.clone();
                        let mut time_working = Duration::new(0, 0);

                        //convert to fold if time
                        for time in work_times {
                            time_idle = time_idle.checked_sub(time).unwrap();
                            time_working = time_working.checked_add(time).unwrap();
                        }

                        let lag: f64 = latency_sleeps.iter().sum();

                        println!("Worker {} terminating. Time alive: {:?}. Time idle: {:?}. Time working: {:?}. Total latency: {}",
                            id,
                            thread_end,
                            time_idle,
                            time_working,
                            lag);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
