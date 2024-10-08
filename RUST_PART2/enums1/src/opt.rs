
pub fn opt() {
  let index = find_first(String::from("Rudra"));

   match index {
    Some(value) => println!("index of a is {}",value),
    None => println!("a not found"),
   }
}

fn find_first (s:String) -> Option<i32> {
    for(index,char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}