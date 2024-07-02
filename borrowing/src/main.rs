fn main() {
    let s1:String=String::from("Hello");
    let s2=&s1; //borrows from s1

    println!("{}",s2);
    println!("{}",s1);  //This is valid , the first pointer wasn't invalidated
  
    borrowing(&s1); //passing the reference of s1
    println!("after{}",s1);
}

fn borrowing(some_string:&String){
println!("{}",some_string);
}