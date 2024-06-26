use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

struct ThreadControl {
    handle: thread::JoinHandle<()>,
    stop_flag: Arc<AtomicBool>,
}

fn spawn_thread(n: usize, stop_flag: Arc<AtomicBool>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("Thread #{} started", n);
        while !stop_flag.load(Ordering::SeqCst) {
            println!("I am thread #{}", n);
            thread::sleep(Duration::from_secs(1));
        }
        println!("Thread #{} stopped", n);
    })
}

fn main() {
    let mut thread_controls = vec![];
    for n in 0..=5 {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let handle = spawn_thread(n, Arc::clone(&stop_flag));
        thread_controls.push(ThreadControl{ handle, stop_flag });
    }

    for control in thread_controls {
        thread::sleep(Duration::from_secs(2));
        control.stop_flag.store(true, Ordering::SeqCst);
        control.handle.join().unwrap();
    }
}
