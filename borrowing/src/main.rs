fn main() {
    let mut s1: String = String::from("Hello");
    let s2 = &s1; //borrows from s1
                  // let s3=& mut s1; //throws an error => Can't have more than one mutable borrower
    let mut s4 = String::from("Hii");
    let s5 = &mut s4;
    // let s6=& s4; 
    
    /*A mutable variable can't have both mutable and immutable references at the same
    time in rust*/

    // println!("{}",s6);

    println!("{}", s5);

    println!("{}", s2);
    println!("{}", s1); //This is valid , the first pointer wasn't invalidated

    borrowing(&s1); //passing the reference of s1
    println!("after{}", s1);

    update_str(&mut s1);
    println!("{}", s1);
}

fn update_str(word: &mut String) {
    word.push_str(" World");
}

fn borrowing(some_string: &String) {
    println!("{}", some_string);
}
