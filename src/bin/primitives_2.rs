fn main() {
    // -------------------------------

    // You can put tuples in tuples!

    let first_tuple = (1, '2', true);
    let second_tuple = ('👾', first_tuple);
    let second_tuple_alternative = ('👾', (1, '2', true));

    // -------------------------------

    // You can make an empty tuple

    let empty_tuple = ();

    // -------------------------------

    // Here, the array has a length of 5 elements and all of them are bound to i32 type signature

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // You can use the .len() method to get the length of an array

    let array_length = array.len(); // 5

    // The & is a "reference" keyword in arrays, which means it is referencing another variable
    // If we put * before it, it dereferences the variable
    // You can "slice" the array like this

    let sliced_array = &array[2..4]; // [3, 4]

    // -------------------------------

    // You can slice a string the same way you would slice an array

    let string = String::from("String!");
    let sliced_string = &string[0..4]; // Stri

    // -------------------------------

    // Concatenating strings together

    let string_1 = String::from("Hello ");
    let string_2 = String::from("world!");
    let concatenated_string = string_1.clone() + &string_2; // Hello world!

    // Don't mind the .clone(), it means you are creating a copy of the string

    // -------------------------------

    println!("First Tuple: {:?}", first_tuple);
    println!("Second Tuple: {:?}", second_tuple);
    println!("Second Tuple Alternative: {:?}", second_tuple_alternative);
    println!("Empty Tuple: {:?}", empty_tuple);
    println!("Array: {:?}", array);
    println!("Array Length: {}", array_length);
    println!("Sliced Array: {:?}", sliced_array);
    println!("String: {}", string);
    println!("Sliced String: {}", sliced_string);
    println!("String 1: {:?}", string_1);
    println!("String 2: {}", string_2);
    println!("Concatenated String: {}", concatenated_string);
}
