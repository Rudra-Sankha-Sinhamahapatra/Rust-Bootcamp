use std::fs::read_to_string;

fn main() {
    let res = read_to_string("a.txt");

    match res {
        Ok(data) => println!("{}",data),
        Err(err) => println!("Error while reading the file {}",err),
    }
    
}
