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
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

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
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, cpu: &Computer) ->
        Worker {

        let thread_start = Instant::now();
        let mut work_times : Vec<Duration> = Vec::new();
        let mut total_latency : Vec<f64> = Vec::new();


        let normal = Normal::new(cpu.mean, cpu.std);
        let time_scale_factor = cpu.work_time_increase_factor.clone();

        let thread = thread::spawn(move ||{
            loop {

                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {

                        let latency = generate_latency(&normal);

                        // measures time working
                        let start_working = Instant::now();

                        job.call_box();

                        extend_time_spent_working(Instant::now().duration_since(start_working), time_scale_factor);

                        work_times.push(Instant::now().duration_since(start_working));
                        total_latency.push(latency);
                    },
                    Message::Terminate => {
                        // worker stat collection and shutdown

                        let thread_end = Instant::now().duration_since(thread_start);
                        let mut time_idle = thread_end.clone();
                        let mut time_working = Duration::new(0, 0);

                        //convert to fold if time
                        for time in work_times {
                            time_idle = time_idle.checked_sub(time).unwrap();
                            time_working = time_working.checked_add(time).unwrap();
                        }

                        let lag: f64 = total_latency.iter().sum();

                        println!("{},{:?},{:?},{:?},{}",
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

fn generate_latency(normal: &Normal) -> f64 {
    let mut latency = normal.sample(&mut rand::thread_rng());
    while latency < 0.0 {
        latency = normal.sample(&mut rand::thread_rng());
    }

    //for testing
    //println!("Sleeping worker {} with normal distributed latency {}", id, latency);

    let secs_to_millisecs = 1000.0;
    let secs = latency.floor();
    let millisecs = (latency - secs) * secs_to_millisecs;

    //thread::sleep(Duration::new(secs as u64, millisecs as u32));

    latency
}

fn extend_time_spent_working(end: Duration, time_scale_factor: f64) {
    let secs = end.as_secs() as f64 * time_scale_factor;
    let nanosecs = end.subsec_nanos() as f64 * time_scale_factor;
    let extra_time = Duration::new(secs as u64, nanosecs as u32);

    thread::sleep(extra_time);
}
