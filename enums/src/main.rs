use std::fs;

enum Direction {
    North,
    East,
    South,
    West,
}

enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(_shape: Shape) -> f64 {
    // calculates the area of the shape
    //pattern matching
    let ans: f64 = match _shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side_length) => side_length * side_length,
    };

    return ans;
}


fn main() {
    let north = Direction::North;
    let south = Direction::South;
    let east = Direction::East;
    let west = Direction::West;

    move_around(north);
    move_around(south);
    move_around(east);
    move_around(west);

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    let circle_area = calculate_area(circle);
    let square_area = calculate_area(square);
    let rectangle_area = calculate_area(rectangle);

    println!("Circle Area:{}", circle_area);
    println!("Square Area:{}", square_area);
    println!("Rectangle Area:{}", rectangle_area);

    //error handling
    let res=fs::read_to_string("example.txt");
   //Similar to Try Catch
    match res{
        Ok(content)=>{
         println!("File Content:{}",content);
        }
        Err(err)=>{
         println!("Error:{}",err);
        }
    }
    println!("Hii There");
    read_from_file_unsafe("ex.txt".to_string());
    println!("Hii There");

    let t1=String::from("Rudra");
    let res=find_first_a(t1);

    match res{
        Some(index)=>println!("Letter 'a' is found at index: {}",index),
        None=>println!("The letter 'a' is not found in the string"),
    }
 
}

fn move_around(_direction: Direction) {}

fn read_from_file_unsafe(_file_content:String)->String{
let res=fs::read_to_string("example.txt");
return res.unwrap();
}

//using option enum 

fn find_first_a(s:String)->Option<i32>{
    for(index,character) in s.chars().enumerate(){
        if character == 'a' {
            return  Some(index as i32);
        }
    }
    return None;
}