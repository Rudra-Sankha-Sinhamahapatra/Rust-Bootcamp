//calculating sum of 1 to 10^8

use std::thread;
use std::sync::mpsc;

pub fn asn1_kirat(){
let (tx,rx) = mpsc::channel();

for i in 0..10 {
    let producer = tx.clone();
    thread::spawn(move || {
        let mut ans:u64 = 0;
        for j in  0..10000000{
            ans = ans + (i * 10000000)+j+1 ;
        }
        producer.send(ans).unwrap();
    });
}

drop(tx);

let mut ans:u64 = 0;

for val in rx {
    ans = ans + val;
    println!("found value");
}
println!("ans is {}",ans);
}