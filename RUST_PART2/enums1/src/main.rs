use enums1::lib;
mod opt;

use opt::opt;

enum Direction {
    North,
    East,
    West,
    South,
}

fn main() {
    let direction = Direction::North;
    move_around(direction);
    let direction2 = Direction::South;
    move_around(direction2);
    let direction3 = Direction::East;
    move_around(direction3);
    let direction4 = Direction::West;
    move_around(direction4);
    lib();
    opt();
}

fn move_around(direction: Direction) {
    match direction {
        Direction::North => println!("North"),
        Direction::South => println!("South"),
        Direction::East => println!("East"),
        Direction::West => println!("West"),
    }
}
