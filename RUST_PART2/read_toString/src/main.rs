use std::fs::read_to_string;
use std::io;

fn main() {
    match read(String::from("a.txt")) {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file: {}", err),
    }
}

fn read(filepath: String) -> Result<String, io::Error> {
   return read_to_string(filepath);  // This returns Result<String, io::Error>
}
