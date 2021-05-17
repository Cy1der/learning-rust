#[allow(dead_code)]
fn main() {
    // A trait is a language feature that tells the Rust compiler about functionality a type must provide.

    // Do you remember the impl keyword, used to call a function with method syntax?

    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    
    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    // Traits are similar, except that we define a trait with just the method signature, then implement the trait for that struct. Like this:

    struct Circle2 {
        x: f64,
        y: f64,
        radius: f64,
    }
    
    trait HasArea {
        fn area(&self) -> f64;
    }
    
    impl HasArea for Circle2 {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    // As you can see, the trait block looks very similar to the impl block, but we don’t define a body, just a type signature. When we impl a trait, we use impl Trait for Item, rather than just impl Item.

    // We can use traits to constrain our generics. Consider this function, which does not compile:

    // fn print_area<T>(shape: T) {
    //     println!("This shape has an area of {}", shape.area());
    // }

    // error: no method named `area` found for type `T` in the current scope

    // Because T can be any type, we can’t be sure that it implements the area method. But we can add a ‘trait constraint’ to our generic T, ensuring that it does:

    fn print_area<T: HasArea>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }
    
    // The syntax <T: HasArea> means any type that implements the HasArea trait. Because traits define function type signatures, we can be sure that any type which implements HasArea will have an .area() method.

    // Extended example:

    trait HasArea2 {
        fn area(&self) -> f64;
    }
    
    struct Circle3 {
        x: f64,
        y: f64,
        radius: f64,
    }
    
    impl HasArea2 for Circle3 {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }
    
    struct Square {
        x: f64,
        y: f64,
        side: f64,
    }
    
    impl HasArea2 for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }
    
    fn print_area2<T: HasArea2>(shape: T) {
        println!("This shape has an area of {}", shape.area());
    }
    
    let c = Circle3 {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area2(c);
    print_area2(s);

    /*
    This shape has an area of 3.141592653589793
    This shape has an area of 1
    */

    // As you can see, print_area2 is now generic, but also ensures that we have passed in the correct types. If we pass in an incorrect type:

    // print_area2(3);

    // error: the trait `HasArea` is not implemented for the type `_` [E0277]

    // So far, we’ve only added trait implementations to structs, but you can implement a trait for any type. So technically, we could implement HasArea for i32:
}