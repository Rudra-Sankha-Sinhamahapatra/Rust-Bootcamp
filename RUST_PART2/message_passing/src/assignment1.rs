//finding sum of 1 to 10^8

use std::thread;
use std::sync::mpsc;

pub fn assignment1() {
let (tx,rx) = mpsc::channel();

thread::spawn(move || {
    let mut partial_sum = 0u64;
    for i in 1..=50_000_000 {
     partial_sum+=i;
    }
    let _=tx.send(partial_sum);
});

let mut total_sum = 0u64;
for i in 50_000_001..=100_000_000 {
    total_sum += i;
}
let value = rx.recv();
match value {
    Ok(partial_sum)=> {
           total_sum+=partial_sum;
        // partial_sum+=total_sum; // when changing this value we have to use mut partial_sum in the scope
        println!("Total sum: {}", total_sum);
    },
    Err(err) => println!("Error receiving data: {}", err),
}
}