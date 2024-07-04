struct Point<T,U>{
    x:T,
    y:U,
    z:U
}

fn main() {
    let integer_point=Point{x:5,y:10,z:8};
    let float_point=Point{x:1.0,y:6.0,z:8.7};
    println!("Integer Point:({},{},{})",integer_point.x,integer_point.y,integer_point.z);
    println!("Float Point:({},{},{})",float_point.x,float_point.y,float_point.z);
}
