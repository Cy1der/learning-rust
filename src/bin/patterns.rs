fn main() {
    // This lesson extends match.rs
    // Patterns are quite common in Rust. We use them in variable bindings, match statements, and other places, too.

    let x: i32 = 1;

    match x {
        1 | 2 => println!("X is 1 or 2"),
        3 => println!("X is 3"),
        _ => println!("X is something else..."),
    }

    // This is pretty self explanatory, you can use elses in match values.

    // Ranges:

    let y: i32 = 3;

    match y {
        1..=5 => println!("Y is 1 through 5"),
        _ => println!("Y is something else..."),
    }

    // 1..=5 (an inclusive range) is the same thing as 1 | 2 | 3 | 4 | 5

    // Ranges are usually used in chars

    let z: char = '😋';

    match z {
        'a'..='j' => println!("Early letter"),
        'k'..='z' => println!("Late letter"),
        _ => println!("Something else..."),
    }

    // Bindings:

    // You can attach values to names

    let a: i32 = 69; // nice

    match a {
        val @ 1..=100 => println!("A is 1 through 100: {}", val),
        _ => println!("A is something else..."),
    }

    // These are useful for complex data structures

    #[derive(Debug)]
    struct Person {
        name: Option<String>,
    }

    let name = "Steve".to_string();
    let x: Option<Person> = Some(Person { name: Some(name) });

    match x {
        Some(Person {
            name: ref a @ Some(_),
            ..
        }) => println!("{:?}", a), // Some("Steve")
        _ => {}
    }

    // Ignoring variants

    // If you’re matching on an enum which has variants, you can use .. to ignore the value and type in the variant:

    enum Integer {
        Value(i32),
        #[allow(dead_code)]
        Missing,
    }

    let y = Integer::Value(10);

    match y {
        Integer::Value(..) => println!("An integer was provided"),
        Integer::Missing => println!("Missing integer"),
    }

    // Gaurds

    // You can introduce ‘match guards’ with if:

    let y = Integer::Value(5);

    match y {
        Integer::Value(i) if i >= 5 => println!("Y is greater than or equal to 5!"),
        Integer::Value(..) => println!("An integer was provided"),
        Integer::Missing => println!("Missing integer"),
    }

    // When using patterns with gaurds, the if will apply to all the values

    let x: i32 = 4;
    let y = false;

    match x {
        4 | 5 if y => println!("Yes!"),
        _ => println!("No."),
    }

    /*
        This prints no, because the if applies to the whole of 4 | 5, and not to just the 5, In other words, the the precedence of if behaves like this:

        (4 | 5) if y => ...

        not this:

        4 | (5 if y) => ...
    */

    // Using references in matches

    let z: i32 = 5;

    match z {
        ref r => println!("Got a reference to {}", r),
    }

    // Mutable References

    let mut a: i32 = 5;

    match a {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    // Destructuring

    // If you have a compound data type, like a struct, you can destructure it inside of a pattern

    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };

    match origin {
        Point { x, y } => println!("({},{})", x, y),
    }

    // We can use : to give a value a different name.

    match origin {
        Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
    }

    // If we only care about some of the values, we don’t have to give them all names:

    match origin {
        Point { x, .. } => println!("X: {}", x),
    }

    // Patterns are very powerful!
    // !!! You can mix and match these concepts! !!!
}
