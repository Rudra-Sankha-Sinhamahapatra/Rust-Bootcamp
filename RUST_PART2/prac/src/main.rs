fn main() {
    let ans = iseven(3);
    println!("{}",ans);
}

fn iseven(num:i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
