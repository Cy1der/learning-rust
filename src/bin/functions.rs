fn main() {
    // Every .rs file needs this function to run something
    // The fn keyword is what tells rust this is a function
    print("Hello World!"); // This is how you call a function
    println!("{}", add_numbers(5, 5));
    println!("{}", early_return());
    diverges();
}

// Components of a function:

/*
1: The name of the function
2: Parameters needed, split by commas, if no parameter is needed, simply use ()
3: The parameter type, THIS IS MANDATORY FOR ALL PARAMETERS
4: The type of data it will return
5: Curly braces which runs the provided code
*/

//   1     2      3   5
fn print(text: &str) {
    println!("{}", text);
}

// Another example
//   1         2   3               4  5
fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y;
}

// You can return a value early
fn early_return() -> i32 {
    return 50; // Anything after a return statement will NOT be run.
               // Assume there is more code here BELOW the return statement
}

/*
Diverging functions
This function will NEVER return a value
Diverging functions can be bound to any type of variable
*/
fn diverges() -> ! {
    panic!("This function never returns!") // panic!() causes the current thread of execution to crash with the given message, similar to println!()
}
