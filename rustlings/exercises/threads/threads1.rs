// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        }
    });

    // Acquiring the lock before the loop deadlocks. The loop never terminates, 
    // because the main thread never releases the lock and the children therefore
    // are never able to complete their work. As such, the while loop never ends
    // and the main thread never releases the lock...
    while status.lock().unwrap().jobs_completed < 10 {
        thread::sleep(Duration::from_millis(500));
        println!("waiting... ");
    }
}
