fn main() {
    // A match statement is like a switch case

    let x: i32 = 5;

    match x {
        // Value => expression
        1 => println!("X is 1"),
        2 => println!("X is 2"),
        3 => println!("X is 3"),
        4 => println!("X is 4"),
        5 => println!("X is 5"),
        6 => println!("X is 6"),
        7 => println!("X is 7"),
        8 => println!("X is 8"),
        9 => println!("X is 9"),
        10 => println!("X is 10"),
        _ => println!("X is something else..."), // If we don't have this arm with the underscore rust will give us an error; error: non-exhaustive patterns: `_` not covered 
    };

    // We can use match to assign values

    let y: i32 = match x {
        1 => 10,
        2 => 9,
        3 => 8,
        4 => 7,
        5 => 6,
        6 => 5,
        7 => 4,
        8 => 3,
        9 => 2,
        10 => 1,
        _ => 0,
    };

    println!("Y is {}", y); // 6

    // The same concepts can be applied to enums
}