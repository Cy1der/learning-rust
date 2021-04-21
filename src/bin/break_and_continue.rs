fn main() {
    // Ending the loop early
    // The "continue" keyword
    // The continue keyword ends the current loop and goes to the next one, here is an example of a loop that only prints ODD numbers

    for number in 0..50 {
        if number % 2 == 0 {
            continue;
        }
        println!("{}", number)
    }

    // The break keyword simply drops the scope and continues the code
    // Example, instead of the done = true nonsense above, we can do this

    for mut x in 5..100 {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            break;
        }
    } // We made the code much cleaner and better
}
