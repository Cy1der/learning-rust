fn main() {
    // While loops are similar to for loops, but go through the code IF the statement returns true

    // Basic while loop

    let mut x: i32 = 5; // The mut keyword means that the variable is mutable (editable), this is not needed in some cases
    let mut done: bool = false;

    while !done {
        // The ! sign before the statement will revert the statement, so if the statement returns false, it will invert it to true and vice versa
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            done = true
        } // ! Make sure to have a guard clause or else the while loop will loop forever!
    } // Here, the loop adds x to itself and subtracts 3, if the number is a multiple of 5, the loop is cancelled

    // If you want an infinite loop, you may be tempted to write this

    /*
    while true {};
    */

    // However, rust has a dedicated keyword for this

    let mut i: i32 = 1;
    loop {
        i += 1;
        println!("{}", i);
        if i == 5000 {
            break; // See break_and_continue.rs for more information
        }
    }

    // For obvious reasons I had to make the infinite loop finite
}
