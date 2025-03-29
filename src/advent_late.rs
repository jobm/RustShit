use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

pub trait SleighTask {
    fn describe(&self) -> String;
}

pub struct SantaSleighQueue {
    record: Mutex<VecDeque<Box<dyn SleighTask + Send>>>, // Use `Send` for thread safety
}

impl SantaSleighQueue {
    // Constructor
    pub fn new() -> Self {
        Self {
            record: Mutex::new(VecDeque::new()),
        }
    }

    // Enqueue method
    pub fn enqueue(&self, task: Box<dyn SleighTask + Send>) {
        self.record.lock().unwrap().push_back(task);
    }

    // Get task method
    pub fn get_task(&self) -> Option<Box<dyn SleighTask + Send>> {
        self.record.lock().unwrap().pop_front()
    }
}

pub struct ElfTask {
    name: String,
    urgency: u32,
}

impl ElfTask {
    pub fn new(name: &str, urgency: u32) -> Self {
        Self {
            name: name.to_string(),
            urgency,
        }
    }
}

impl SleighTask for ElfTask {
    fn describe(&self) -> String {
        format!("Elf task: {} (urgency {})", self.name, self.urgency)
    }
}

pub struct ReindeerTask {
    name: String,
    weight: u32,
}

impl ReindeerTask {
    pub fn new(name: &str, weight: u32) -> Self {
        Self {
            name: name.to_string(),
            weight,
        }
    }
}

impl SleighTask for ReindeerTask {
    fn describe(&self) -> String {
        format!("Reindeer task: {} ({} kg)", self.name, self.weight)
    }
}

pub fn main() {
    let queue = Arc::new(SantaSleighQueue::new());

    let producer_queue = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        producer_queue.enqueue(Box::new(ReindeerTask::new("Deliver Toys", 100)));
        producer_queue.enqueue(Box::new(ElfTask::new("Wrap Gifts", 3)));
        producer_queue.enqueue(Box::new(ReindeerTask::new("Collect Reindeer Feed", 50)));
        producer_queue.enqueue(Box::new(ElfTask::new("Decorate Tree", 7)));
    });

    thread::sleep(std::time::Duration::from_millis(10));

    let consumer_queue = Arc::clone(&queue);
    let consumer = thread::spawn(move || loop {
        if let Some(task) = consumer_queue.get_task() {
            println!("{}", task.describe());
        } else {
            break;
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
