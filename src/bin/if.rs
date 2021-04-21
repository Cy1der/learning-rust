fn main() {
    // If statements are pretty simple

    // Basic if statement

    let x: i32 = 5;

    if x == 5 {
        println!("X is 5!");
    }

    // If the value of X was anything else it would not print "X is 5!"

    // Same thing but with else

    let y: i32 = 1;

    if y == 5 {
        println!("Y is 5!");
    } else {
        println!("Y is not 5.");
    }

    // If you want multiple if cases

    if y == 5 {
        println!("Y is 5!");
    } else if y == 1 {
        println!("Y is 1!");
    } else {
        println!("Y is not 5 or 1.");
    }

    // Using if statements to have dynamic values

    let z: i32 = 10;
    let dynamic_value = if z == 5 { 50 } else { 100 }; // dynamic_value will have a value of 100 because z is not 50;

    println!("Dynamic Value: {}", dynamic_value)
}
