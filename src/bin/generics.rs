#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    // Sometimes, when writing a function or data type, we may want it to work for multiple types of arguments. In Rust, we can do this with generics. Generics are called ‘parametric polymorphism’ in type theory, which means that they are types or functions that have multiple forms (‘poly’ is multiple, ‘morph’ is form) over a given parameter (‘parametric’).
    // Rust’s standard library provides a type, Option<T>

    // enum Option<T> {
    //     Some(T),
    //     None
    // }

    // The <T> part, which you’ve seen a few times before, indicates that this is a generic data type. Inside the declaration of our enum, wherever we see a T, we substitute that type for the same type used in the generic. Here’s an example of using Option<T>, with some extra type annotations:

    let x: Option<i32> = Some(10);

    // In the type declaration, we say Option<i32>. Note how similar this looks to Option<T>. So, in this particular Option, T has the value of i32. On the right-hand side of the binding, we do make a Some(T), where T is 5. Since that’s an i32, the two sides match, and Rust is happy. If they didn’t match, we’d get an error:

    // let x: Option<f64> = Some(5); // error: mismatched types: expected `core::option::Option<f64>`, found `core::option::Option<_>` (expected f64 but found integral variable)

    // That doesn’t mean we can’t make Option<T>s that hold an f64! They just have to match up:

    let y: Option<i32> = Some(5);
    let z: Option<f64> = Some(5.0f64);

    // This is just fine. One definition, multiple uses.

    // Generics don’t have to only be generic over one type. Consider another type from Rust’s standard library that’s similar, Result<T, E>:

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    
    // This type is generic over two types: T and E. By the way, the capital letters can be any letter you’d like. We could define Result<T, E> as:

    // enum Result<A, Z> {
    //     Ok(A),
    //     Err(Z),
    // }

    // If we wanted to. Convention says that the first generic parameter should be T, for ‘type’, and that we use E for ‘error’. Rust doesn’t care, however.
    // The Result<T, E> type is intended to be used to return the result of a computation, and to have the ability to return an error if it didn’t work out.

    println!("{:?} {:?} {:?}", x, y, z);

    // GENERIC FUNCTIONS

    // We can write functions that take generic types with a similar syntax:

    fn takes_anything<T>(x: T) {
        // do something with x
    }

    // The syntax has two parts: the <T> says “this function is generic over one type, T”, and the x: T says “x has the type T.”

    // Multiple arguments can have the same generic type:

    fn takes_two_of_the_same_things<T>(x: T, y: T) {
        // ...
    }

    // We could write a version that takes multiple types:

    fn takes_two_things<T, U>(x: T, y: U) {
        // ...
    }

    // Generic functions are most useful with ‘trait bounds’

    // GENERIC STRUCTS

    // You can store a generic type in a struct as well:

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_origin = Point { x: 0, y: 0 };
    let float_origin = Point { x: 0.0, y: 0.0 };

    // Similarly to functions, the <T> is where we declare the generic parameters, and we then use x: T in the type declaration, too.

    println!("{:?} {:?}", int_origin, float_origin);
}