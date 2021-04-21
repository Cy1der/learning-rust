fn main() {
    // ! Naming your variables/functions etc in snake case is recommended

    /*
    Integer Types
    i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64

    F prefix is a floating point number, 64 is used for double precision, and 32 is used for single precision.
    I prefix means the variable is signed.
    U prefix means the variable is unsigned.

    The number is the amount of bytes it takes up in memory.
    Usize and isize are dynamic, they depend on the Operating System, if the OS is 32 bit, usize and isize will be u32 and i32. Same thing for 64 bit OS's.
    */

    // Math Operations

    let addition = 5 + 5;
    let subtraction = 5 - 5;
    let multiplication = 5 * 5;
    let division = 5 / 5;
    let remainder = 5 % 5;

    /*
    Other Variable Types

    Boolean: bool (Can be either true or false)
    Character: char (Can be a single charcater, for example 'x', any character is accepted, including emojis and other languages)
    */

    let boolean: bool = true;
    let character: char = '😃';

    // -------------------------------

    // Using a string type
    // To assign a variable to type string, it is a little different

    let string = String::from("Hello world!");
    let string_alternative = "Hello world!".to_string();

    // -------------------------------

    // Tuples (Collections of data, does not have to be the same type)

    let tuple: (i32, f64, char) = (15, 10.01, '😎');

    // Destructuring tuples

    let (x, y, z) = tuple; //x = 15, y = 10.01, z = "😎"
    let (_, _, emoji) = tuple; // If you want a specific piece of data from the tuple, use underscores when destructuring, the correspoinding piece of data will be ignored. In this case, emoji = '😎' and thats it
    let float_number = tuple.1; // You can call a piece of data from a tuple without destructuring by doing tuple.<index>. Here, floatNumber = 10.01

    // -------------------------------

    // Arrays

    // Arrays are like tuples but can only be one single type, used for long lists

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // To call a single piece of data from the array, use array[<index>]

    let array_first = array[0];

    // Arrays are useful for having data allocated on the stack and not on the heap or to ensure you have a fixed number of elements

    // -------------------------------

    // ! Note that arrays and tuples are zero indexed!

    println!("Addition: {}", addition);
    println!("Subtraction: {}", subtraction);
    println!("Multiplication: {}", multiplication);
    println!("Division: {}", division);
    println!("Remainder: {}", remainder);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("X: {}", x);
    println!("Y: {}", y);
    println!("Z: {}", z);
    println!("Emoji: {}", emoji);
    println!("Float Number: {}", float_number);
    println!("Array: {:?}", array);
    println!("Tuple: {:?}", tuple);
    println!("Array First: {}", array_first);
    println!("String: {}", string);
    println!("String Alternative: {}", string_alternative);
}
