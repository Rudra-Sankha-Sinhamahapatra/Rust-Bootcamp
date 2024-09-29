
fn get_string_length (s:&str) -> usize {
return s.chars().count();
}


fn main() {
    let mystring = String::from("Walk down the hook");
    let length = get_string_length(&mystring);
    println!("{}",length);
}


