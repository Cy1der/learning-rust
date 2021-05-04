#[allow(dead_code)]
fn main() {
    // This is fine
    // baz(bar(foo));

    // But we can make it better using methods!
    // baz(bar(foo)) is the SAME thing as foo.bar().baz();

    // Methods are much more readable and are more efficient

    // To bind methods to values you use the "impl" syntax (You might have seen this word in other lessons)

    // Here's how it works

    #[derive(Debug)]
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }

    // The name of the IMPL must be the same thing as the struct name or enum name

    impl Circle {
        // 1      2        3  4
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn grow(&self, increment: f64) -> Circle {
            Circle {
                x: self.x,
                y: self.y,
                radius: self.radius + increment,
            }
        }
    }

    // You can attach multiple methods (no need to add commas between each method)

    /*
    1: Method name, prefixed with fn
    2: Always add &self (a reference to the bound value), include any other values you might need
    3: The type of value it returns (Optional)
    4: Execution of code
    */

    let c = Circle {
        x: 0.0,
        y: 0.0,
        radius: 2.0,
    };
    println!("Area of circle: {} units", c.area()); // 12.566370614359172

    // Method chaining [foo.bar().baz()]

    let c2 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
    };
    println!("Area of circle 2: {} units", c2.area()); // 78.53981633974483
    let d = c.grow(10.0).area();
    println!("Area of grown circle: {} units", d); // 452.3893421169302

    // Associated Functions (do not take self parameter)

    // This example is very common in rust-lang

    impl Circle {
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle { x, y, radius }
        }
    }

    let c3 = Circle::new(0.0, 0.0, 6.0);
    println!("Circle 3: {:?}", c3); // Circle { x: 0.0, y: 0.0, radius: 6.0 }
}
