fn main() {
    println!("{}",is_even(20));
}

fn is_even(num:u32) -> bool {
 if num % 2 ==0 {
    return true;
 }
 return false;
}
