struct User1 <'a,'b> {
first_name: &'a str,
last_name: &'b str,
}

pub fn struct2() {
let user:User1;
let first_name = String::from("Rudra");
{
    let last_name = String::from("Sankha");
    user = User1 {first_name:&first_name,last_name:&last_name};
    println!("The first name of the user is {}",user.first_name);
    println!("The last name of the user is {}",user.last_name);
}
// println!("The first name of the user is {}",user.first_name); //`last_name` does not live long enough
// println!("The last name of the user is {}",user.last_name); //`last_name` does not live long enough
}