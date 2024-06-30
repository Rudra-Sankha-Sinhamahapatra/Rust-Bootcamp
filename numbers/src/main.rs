fn main() {
    let mut x: i64 = -500000;
    for i in 0..1000 {
        x = x+i + 100;
    }

    println!("{}", x);
}
