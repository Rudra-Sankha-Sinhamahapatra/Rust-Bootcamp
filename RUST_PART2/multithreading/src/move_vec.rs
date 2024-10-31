use std::thread;

pub fn move_vec() {
  let v = vec![1,2,3];
  let handler = thread::spawn(move || {
    println!("{:?}",v);
  });

  handler.join().unwrap();
   /*as v is moved to the spawn thread so we are waiting for 
  it before thread finishes otherwise it doesn't print anything as v goes out of scope*/
}