// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // Lock the mutex before accessing the shared value
            let mut num_completed = status_shared.lock().unwrap();
            num_completed.jobs_completed += 1;
            // The lock is automatically released when `num_completed` goes out of scope
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Lock the mutex to access the shared value
    let num_completed = status.lock().unwrap();
    println!("jobs completed {}", num_completed.jobs_completed);
}