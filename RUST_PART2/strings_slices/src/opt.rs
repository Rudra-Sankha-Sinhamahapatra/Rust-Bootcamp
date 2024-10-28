use std::io::{self, Write};

pub fn opt () {
println!("Welcome to String slice utility");
println!("Enter a string");

io::stdout().flush().unwrap();//prompt text should appear before the user input

let mut input_string = String::new();

io::stdin().read_line(&mut input_string).expect("Failed to read");

let input_string = input_string.trim();

let start_index = match get_index("Enter the start index: "){
Some(index)=>index,
None=>return,
};

let last_index = match get_index("Enter the last index: ") {
Some(index) => index,
None => return,
};

if start_index >= last_index || last_index >input_string.len() {
    println!("Invalid range ensure 0 <- start < end <- {}",input_string.len());
    return;
}

let slice = &input_string[start_index..last_index];
    println!("Your slice: \"{}\"", slice);

}

pub fn get_index(prompt:&str) -> Option<usize> {
    println!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse::<usize>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Invalid input,please enter a positive integer");
            None
        }
    }
}