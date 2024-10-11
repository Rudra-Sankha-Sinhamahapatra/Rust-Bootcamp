fn main() {
    let ans = iseven(3);
    println!("{}",ans);
    let p = 5;
    // let p1 = p;
    println!("{}",p);
    let s = String::from("h");
    let s1 = s;
    // println!("{}",s);
    let s2 = &s1;
    println!("{}",s2);
}

fn iseven(num:i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
