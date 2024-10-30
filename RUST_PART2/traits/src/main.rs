
trait Summary {
    fn summarise (&self) -> String {
        return  String::from("Hello from trait default");
    }
}

trait Fixi {
    fn fix(& self) -> String {
        return String::from("From Fixi trait");
    }
}

struct User {
    name:String,
    age:u32,
}

struct Fix;
impl Summary for Fix {}

impl Summary for String {}

impl Summary for User {
fn summarise (&self) -> String {
    return format!("The name is {}, and the age is {}",self.name.clone(),self.age);
}
}

impl Fixi for User {}

fn main() {
    let user = User {
        name:String::from("Rudra Sankha Sinhamahapatra"),
        age:20
    };
    let f = Fix;
    notify(user);
    notify(String::from("Hello"));
    notify(f);
    let h = Fix;
    notify2(&h);
    let user2 = User {
        name:String::from("Akash Bhardwaj"),
        age:22
    };

    notify3(user2);
}

fn notify(u: impl Summary) {
    println!("{}",u.summarise());
}

fn notify2<T:Summary>(item:&T){
    println!("{}",item.summarise());
}

fn notify3<T:Summary + Fixi>(item:T){
    println!("{}",item.summarise());
    println!("{}",item.fix());
}