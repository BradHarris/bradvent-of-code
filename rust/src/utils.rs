use std::time::Duration;

pub fn clear_terminal() {
    print!("{esc}c", esc = 27 as char);
    print!("\x1b[{};{}H", 0, 0);
}

#[derive(Debug)]
pub struct DayPerfMetric {
    pub day: usize,
    pub part1: Duration,
    pub part2: Duration,
}

pub fn print_time_results(results: Vec<DayPerfMetric>, runs: usize) {
    println!();
    println!("+{:-^38}+", format!("averaged over {runs} runs"));
    println!("| {: <6} | {: <12} | {: <12} |", "day", "part 1", "part 2");
    let total = results
        .iter()
        .inspect(|DayPerfMetric { day, part1, part2 }| {
            println!(
                "| {day: <6} | {: <12} | {: <12} |",
                format!("{part1:?}"),
                format!("{part2:?}")
            )
        })
        .map(|m| m.part1 + m.part2)
        .sum::<Duration>();
    println!("+{:-^38}+", format!("total: {total:?}"));
    println!();
}

// taken directly from the rust book...

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            #[cfg(debug_assertions)]
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
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    #[cfg(debug_assertions)]
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    #[cfg(debug_assertions)]
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
