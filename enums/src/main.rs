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
}

fn move_around(_direction: Direction) {}
