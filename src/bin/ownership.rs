fn main() {
    let x = 1; // Here, x owns 1

    // Each piece of data can have 1 owner at a time
    // When the owner goes out of scope, the value will be dropped
    // Example:

    let y = x;

    {
        let a = 10;
        println!("A: {}", a);
    }

    // x + a; We get a error because A is not defined in the scope we are currently in, for this reason, the code will be commented out
    // If we take A and put it in the scope we are in, it will work

    // Example 2:

    let string = String::from("String!");
    let string_2 = string;

    // println!("{}", string); Only 1 reference can own a piece of data at a time! So here we get a error, for this reason, the code will be commented out

    // To fix such error, we "borrow" the reference

    let string_3 = String::from("String...");
    let string_4 = &string_3;
    //             ^

    println!("String 3: {}", string_3); // This will work!

    println!("X: {}", x);
    println!("Y: {}", y);
    println!("String 2: {}", string_2);
    println!("String 4: {}", string_4);
}
