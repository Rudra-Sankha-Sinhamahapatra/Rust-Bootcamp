use std::collections::HashMap;

fn main() {
  let mut users = HashMap::new();

  users.insert(String::from("Rudra"), 20);
  users.insert(String::from("Priyant"), 21);
  users.insert(String::from("Anshuman"), 19);

  println!("{:?}",users);

  let first_user = users.get("Rudra");
  
  match first_user {
    Some(age) => println!("age is {:?}",age),
    None => println!("user is noy present in db"),
  }
}
