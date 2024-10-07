use struct1::rectarea;
struct User {
    first_name:String,
    last_name: String,
    age: i32
}

fn main() {
    let user = User {
        first_name:String::from("Rudra Sankha"),
        last_name:String::from("Sinhamahapatra"),
        age:20
    };

    println!("{}",user.first_name);
    println!("{}",user.last_name);
    println!("{}",user.age);
    
    rectarea();
}

