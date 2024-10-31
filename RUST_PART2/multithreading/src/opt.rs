use std::thread;
use std::time::Duration;

pub fn opt() {
  thread::spawn(||{
        for i in 0..5 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

}

// Output
/*
hi number 1 from the main thread
hi number 1 from the spawned thread!
hi number 2 from the main thread
hi number 2 from the spawned thread!
hi number 3 from the main thread
hi number 3 from the spawned thread!
hi number 4 from the main thread
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
*/