#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    println!("up: {:?}", up);
    println!("down: {:?}", down);
    println!("left: {:?}", left);
    println!("right: {:?}", right);
}
