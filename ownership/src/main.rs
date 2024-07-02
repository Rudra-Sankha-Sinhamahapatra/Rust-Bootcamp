
fn main() {
    //stack variables=>Multi owner
   let l1=2;
   let l2=l1;
   println!("{}",l1);
   println!("{}",l2);

    //heap variables=>Single owner
    let s1=String::from("Hii");
    let s2=s1;
    println!("{}",s2);
    // println!("{}",s1); //s1 isnt valid

    //the borrower can also take back the ownership
    let mut my_string=String::from("Hello");
    my_string=takes_ownership(my_string);
     
    // println!("{}"),my_string;//will throw error because some_string took the ownership of my_string
    //solution=> we have to make a clone of my_string
    println!("{}",my_string);

    let jk=String::from("Hello2");
    takes_ownership2(jk.clone());
    println!("{}",jk);

}


//the borrower can also take back the ownership
fn takes_ownership(some_string:String)->String{
    println!("{}",some_string);
    return some_string;
    //some_string now owns the data 
}

fn takes_ownership2(some_string2:String){
    println!("{}",some_string2);
    //some_string now owns the data 
}