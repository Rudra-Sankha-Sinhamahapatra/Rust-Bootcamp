use std::fmt::Debug;

struct User {
    active: bool,
    id: u8,
    name: String,
    email: String,
    signin_count: u64,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        self.width + self.height
    }
}

struct Square {
    side: u64,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the square prints:{}", self.side * self.side)
    }
}

struct Hello;

impl Hello {
    fn hello(&self) -> String {
        "Hello World".to_string()
    }
}

fn main() {
    let user = User {
        active: true,
        id: 1,
        name: String::from("Adam Smith"),
        email: String::from("adam@gmail.com"),
        signin_count: 578,
    };

    println!("User Details =>");
    println!("Active:{}", user.active);
    println!("id:{}", user.id);
    println!("Name:{}", user.name);
    println!("Email:{}", user.email);
    println!("Signin Count:{}", user.signin_count);

    let rect = Rect {
        height: 30,
        width: 30,
    };

    println!("Area of the Rectangle is :{}", rect.area());
    println!("Perimeter of the Rectangle is :{}", rect.perimeter());

    let square = Square { side: 8 };

    println!("All about Square:{:?}", square);

    let v1 = Hello;
    println!("{}", v1.hello())
}
