// Basic enum

// Camel case is recommended for naming enums
#[derive(Debug)]
enum Direction {
    Up(Coordinates),
    Down(Coordinates),
    Left(Coordinates),
    Right(Coordinates), // You can have tuples and structs in enums
}

#[derive(Debug)]
enum Keys {
    UpKey(String),
    DownKey(String),
    LeftKey(String),
    RightKey(String),
}

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Direction {
    // We can use the impl keyword to attach methods to our enum
    fn match_direction(&self) -> Keys {
        match *self {
            Direction::Up(_) => Keys::UpKey(String::from("Up key")),
            Direction::Down(_) => Keys::DownKey(String::from("Down key")),
            Direction::Left(_) => Keys::LeftKey(String::from("Left key")),
            Direction::Right(_) => Keys::RightKey(String::from("Right key")),
        }
    }
}

impl Keys {
    fn destruct(&self) -> &String {
        match *self {
            Keys::UpKey(ref s) => s,
            Keys::DownKey(ref s) => s,
            Keys::LeftKey(ref s) => s,
            Keys::RightKey(ref s) => s,
        }
    }
}

fn main() {
    // An enum in Rust is a type that represents data that could be one of several possible variants

    // This is how we instantiate the direction
    let up = Direction::Up(Coordinates { x: 0, y: 1 });
    let down = Direction::Down(Coordinates { x: 0, y: -1 });
    let left = Direction::Left(Coordinates { x: -1, y: 0 });
    let right = Direction::Right(Coordinates { x: 1, y: 0 });

    println!("{:?}", up); // Up(Coordinates { x: 0, y: 1 })
    println!("{:?}", down);
    println!("{:?}", left);
    println!("{:?}", right);

    let up_2 = up.match_direction();
    let down_2 = down.match_direction();
    let left_2 = left.match_direction();
    let right_2 = right.match_direction();

    println!("{:?}", up_2); // UpKey("Up key")
    println!("{:?}", down_2);
    println!("{:?}", left_2);
    println!("{:?}", right_2);

    let up_3 = up_2.destruct(); // "Up key"
    let down_3 = down_2.destruct();
    let left_3 = left_2.destruct();
    let right_3 = right_2.destruct();

    println!("{:?}", up_3);
    println!("{:?}", down_3);
    println!("{:?}", left_3);
    println!("{:?}", right_3);
}
