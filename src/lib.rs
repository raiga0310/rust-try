use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>
}

struct Worker {
	id: usize,
	thread: thread::JoinHandle<()>,
}

type Job = Box<FnOnce() + Send + 'static>;

impl ThreadPool {
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
			sender,
		}
	}

	pub fn spawn<F, T>(f: F) -> JoinHandle<T>
		where
			F: FnOnce() -> T + Send + 'static,
			T: Send + 'static
	{

	}

	pub fn excute<F> (&self, f: F)
		where
			F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);

		self.sender.send(job).unwrap();
	}
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let job = receiver
					.lock()
					.unwrap()
					.recv()
					.unwrap();

				println!("Worker {} got a job; executeing.", id);

				(*job)();
			}
		});

		Worker {
			id,
			thread,
		}
	}
}
}