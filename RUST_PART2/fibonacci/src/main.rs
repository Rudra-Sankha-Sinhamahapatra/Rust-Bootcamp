fn main() {
    println!("{}",fib(10));
}

fn fib(num:u32) -> u32 {
 let mut a = 0;
 let mut b = 1;

  if num == 1 || num == 0 {
    return num;
  }


  for _ in 0..(num - 1) {
    let t = b;
    b = b+a;
    a = t;
  }

  return b;
}