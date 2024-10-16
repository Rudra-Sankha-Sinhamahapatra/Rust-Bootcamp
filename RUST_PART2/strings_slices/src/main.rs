use strings_slices::lib;

fn main() {
    let word = String::from("hello");
    let word2 = &word[0..3];

    println!("{}",word2);

    let str = String::from("Hello This is my first day");
    println!("{}",find_first_word(&str));
    
    lib();
}

fn find_first_word (str:&String)-> &str {
let mut space_index = 0;
for i in str.chars() {
    if i == ' ' {
        break;
    }
    space_index +=1;
}
 return &str[0..space_index];
}