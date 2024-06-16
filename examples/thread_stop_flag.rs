use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct ThreadControl {
    handle: thread::JoinHandle<()>,
    stop_flag: Arc<AtomicBool>,
}

fn main() {
    let mut thread_controls = vec![];
    for n in 0..=5 {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let stop_flag_clone: Arc<AtomicBool> = Arc::clone(&stop_flag);
        let handle = thread::spawn(move || {
            println!("Thread {n} started");
            while !stop_flag_clone.load(Ordering::SeqCst) {
                println!("{n}");
                thread::sleep(Duration::from_secs(1));
            }
            println!("Thread {n} stopped")
        });
        thread_controls.push(ThreadControl{ handle, stop_flag });
    }

    for control in thread_controls {
        thread::sleep(Duration::from_secs(2));
        control.stop_flag.store(true, Ordering::SeqCst);
        control.handle.join().unwrap();
    }
}
