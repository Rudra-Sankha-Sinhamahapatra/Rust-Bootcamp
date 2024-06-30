
pub mod c;

fn main() {
   let  greeting= String::from("Hello World!");
   println!("{}",greeting);
   let char1=greeting.chars().nth(0);

match char1{
 Some(c)=>println!("{}",c),
 None=>println!("No character at this index"),
}

   c::c();
}
