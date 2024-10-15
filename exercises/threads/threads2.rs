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
            // Lock the mutex before modifying the shared value
            let mut num_completed = status_shared.lock().unwrap();
            num_completed.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Print the value of JobStatus.jobs_completed after all threads have completed
    let final_count = {
        let num_completed = status.lock().unwrap();
        num_completed.jobs_completed
    };
    println!("jobs completed {}", final_count);
}