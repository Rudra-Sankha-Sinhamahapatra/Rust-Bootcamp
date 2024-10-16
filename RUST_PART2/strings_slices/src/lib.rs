
pub fn lib() {
    let str = "Hello";
    println!("{}",str);

    let vec = vec![1,2,3];
    let vec2 = &vec[1..=2];
    println!("{:?}",vec2);
}