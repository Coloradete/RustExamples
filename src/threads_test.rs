use std::thread;
use std::time;

pub fn call_thread_test(){

    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    for _ in 1..10 {
        print!("-");
        thread::sleep(time::Duration::from_millis(10));
    }

    handle.join();
}