use std::sync::mpsc;
use std::thread;

pub fn lib() {
   let (tx,rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("hi");
        let _ = tx.send(val);
    });

    let value = rx.recv();
    match value {
        Ok(value)=>println!("Got:{}",value),
        Err(err)=>  println!("Error:{}",err),
    }
    
}
