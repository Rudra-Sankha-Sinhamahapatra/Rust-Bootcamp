use std::thread;

pub fn spawn() {
let handle =  thread::spawn(||{
        for i in 0..5 {
            println!("hi number {i} from the spawned thread!");
        }
    });

    for i in 0..50 {
        println!("hi number {i} from the main thread");
    }
    let _ = handle.join(); //we wait until the spawn thread runs, (joining the spawned thread with main)

}

//Output:
/*
hi number 0 from the main thread
hi number 1 from the main thread
hi number 2 from the main thread
hi number 3 from the main thread
hi number 4 from the main thread
hi number 0 from the spawned thread!
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
*/