/*
"Lending out a reference to a resource that someone else owns can be complicated. For example, imagine this set of operations:

    - I acquire a handle to some kind of resource.
    - I lend you a reference to the resource.
    - I decide I’m done with the resource, and deallocate it, while you still have your reference.
    - You decide to use the resource.

Uh oh! Your reference is pointing to an invalid resource. This is called a dangling pointer or ‘use after free’, when the resource is memory.

To fix this, we have to make sure that step four never happens after step three. The ownership system in Rust does this through a concept called lifetimes, which describe the scope that a reference is valid for."
*/

fn main() {
    let variable = Variable { x: &10 };

    println!("{}", implicit(&5));
    println!("{}", explicit(&5));
    println!("{}", variable.x);
    println!("{} {} {}", STRING, FOO, FOO_X);
    x_or_y("1", "2");
    x_or_y_2("1", "2");
}

// Implicit lifetimes
fn implicit(x: &i32) -> i32 {
    return *x;
}

// Explicit lifetimes
fn explicit<'a>(x: &'a i32) -> i32 {
    return *x;
}

// The 'a means "lifetime a"

// Explanation for explicit:

/*
The <> declares our lifetimes, the explicit function has one lifetime of 'a.
If we wanted more than one lifetime, we would do something like this: <'a,'b>

In the parameter list, we use the lifetimes we named
(x: &'a i32)

If we wanted a mutable reference, we'd do this:
(x: &'a mut i32)

If you compare &mut i32 to &'a mut i32, they’re the same, it’s just that the lifetime 'a has snuck in between the & and the mut i32. We read &mut i32 as ‘a mutable reference to an i32’ and &'a mut i32 as ‘a mutable reference to an i32 with the lifetime 'a’.
*/

// We also use lifetimes in structs (See structs.rs for more information!)

// Basic example

struct Variable<'a> {
    x: &'a i32, // So why do we need a lifetime here? We need to ensure that any reference to a Foo cannot outlive the reference to an i32 it contains.
}

// If you have multiple references, you can use the same lifetime multiple times

fn x_or_y<'a>(x: &'a str, y: &'a str) {
    println!("{} {}", x, y)
} // This says that x and y both are alive for the same scope, and that the return value is also alive for that scope.

//If you wanted x and y to have different lifetimes, you can use multiple lifetime parameters

fn x_or_y_2<'a, 'b>(x: &'a str, y: &'b str) {
    println!("{} {}", x, y)
} // In this example, x and y have different valid scopes, but the return value has the same lifetime as x.

// -----------------------------------

// The 'static lifetime

// The lifetime named ‘static’ is a special lifetime. It signals that something has the lifetime of the entire program. Most Rust programmers first come across 'static when dealing with strings

// Example

const STRING: &'static str = "Hello World!";

// String literals have the type &'static str because the reference is always alive: they are baked into the data segment of the final binary.

// Another example are globals

const FOO: i32 = 5; // Const variables don't change
const FOO_X: &'static i32 = &FOO;

// This adds an i32 to the data segment of the binary, and x is a reference to it.
