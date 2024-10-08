pub enum Shape {
    Rectangle(f64, f64), //width & height
    Circle(f64),         //radius
}
pub fn lib() {
    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);
    let cir = Shape::Circle(7.0);
    calculate_area(cir);
}

fn calculate_area(shape: Shape) {
    //pattern matching
    match shape {
        Shape::Rectangle(a, b) => println!("area of shape Rectangle is {}", a * b),
        Shape::Circle(r) => println!("area of shape Circle is {}", 3.14 * r * r),
    };
}
