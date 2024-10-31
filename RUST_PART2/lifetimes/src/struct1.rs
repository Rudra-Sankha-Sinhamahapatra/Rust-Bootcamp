
struct User<'a> {
    name: &'a str,
}

pub fn struct1() {
   let name = String::from("Rudra");
   let user = User {
    name:&name,
   };
   println!("The name of the user is {}",user.name);
}
