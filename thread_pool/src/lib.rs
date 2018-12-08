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
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, cpu: &Computer) ->
        Worker {

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
                        println!("Sleeping worker {} with normal distributed latency {}", id, latency);

                        // Split latency into seconds and milliseconds required by Duration
                        let secs_to_millisecs = 1000.0;
                        let mut secs = latency.floor();
                        let mut millisecs = (latency - secs) * secs_to_millisecs;
                        thread::sleep(Duration::new(secs as u64, millisecs as u32));

                        let start = Instant::now();
                        job.call_box();

                        let end = Instant::now().duration_since(start);
                        secs = end.as_secs() as f64 * time_scale_factor;
                        millisecs = end.subsec_millis() as f64 * time_scale_factor;
                        let extra_time = Duration::new(secs as u64, millisecs as u32);
                        thread::sleep(extra_time);
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

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
