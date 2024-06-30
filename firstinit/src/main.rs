fn main() {
    println!("Hello, world!");
    let mut x: i64 = -500000;
    let y: u32 = 1000;
    let z: f32 = 100.109;
  
    for i in 0..1000 {
        x = i + 100;
    }

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
