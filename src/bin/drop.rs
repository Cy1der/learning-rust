#[allow(unused_variables)]
fn main() {
    // Now that we’ve discussed traits, let’s talk about a particular trait provided by the Rust standard library, Drop. The Drop trait provides a way to run some code when a value goes out of scope.

    struct HasDrop;

    impl Drop for HasDrop {
        fn drop(&mut self) {
            println!("Dropping!");
        }
    }

    let x = HasDrop;

    // When x goes out of scope at the end of main(), the code for Drop will run. Drop has one method, which is also called drop(). It takes a mutable reference to self.
    //The mechanics of Drop are very simple, but there are some subtleties. For example, values are dropped in the opposite order they are declared. 

    // Another example:

    struct Firework {
        strength: i32,
    }
    
    impl Drop for Firework {
        fn drop(&mut self) {
            println!("BOOM x {}!!!", self.strength);
        }
    }
    
    let firecracker = Firework { strength: 1 };
    let tnt = Firework { strength: 100 };

    // The TNT goes off before the firecracker does, because it was declared afterwards. Last in, first out.

    // So what is Drop good for? Generally, Drop is used to clean up any resources associated with a struct. For example, the Arc<T> type is a reference-counted type. When Drop is called, it will decrement the reference count, and if the total number of references is zero, will clean up the underlying value.
} // x goes out of scope here


// The console prints this

/*
BOOM x 100!!!
BOOM x 1!!!
Dropping!
*/