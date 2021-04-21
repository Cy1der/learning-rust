fn main() {
    // You know what a loop is, right!?

    /*
    Loops in rust are NOT C-Style!
    The format looks like this:

    for var in expression {}
    */

    // Basic loop

    for i in 0..10 {
        // This loop will loop 10 times, the value will start from 0 and ends at 9
        println!("{}", i)
    }

    // To keep track of how many times you already looped, you can use the .enumerate() method like this

    for (x, y) in (5..10).enumerate() {
        // x is how many times it has looped (zero indexed!) and y is the value
        println!("x = {} | y = {}", x, y);
    }
}
