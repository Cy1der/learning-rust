// Components of a struct

/*
1: The keyword
2: The name of the struct (Must be capitalized!)
3: Curly brackets to scope
4: The data entry name
5: The data entry type
*: This is found in a different guide (Lifetimes)
*/

// 1     2   *    3
struct User<'a> {
    //4    *   5
    name: &'a str,
    age: i16,
}

fn main() {
    // Structs are a way of creating more complex data types, like interfaces from Typescript
    // Basic example (Declare structs outside the main function)
    let user = User {
        name: "Cy1der",
        age: 14,
    };

    println!("User Age: {}\nUser Name: {}", user.age, user.name); // You can call the data separately just like Javascript Objects
}
