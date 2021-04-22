fn main() {
    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) { // A vec (vector) acts EXACTLY like an array, but is resizable
        // Do stuff with v1 and v2
        // Hand back ownership, and the result of our function
        return (v1, v2, 42);
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let (v1, v2, answer) = foo(v1, v2);

    // This is some pretty nasty code as it does not take advantage of borrowing
    // We can do something like this
    
    fn foo_2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // Do stuff with v1 and v2
        // Return the answer
        println!("{:?}, {:?}", v1, v2);
        return 42;
    }

    let v1_2 = vec![1, 2, 3];
    let v2_2 = vec![4, 5, 6];
    let answer_2 = foo_2(&v1, &v2);
    
    // We can use v1_2 and v2_2 here!
    // Instead of taking Vec<i32>s as our arguments, we take a reference: &Vec<i32>. And instead of passing v1_2 and v2_2 directly, we pass &v1_2 and &v2_2. We call the &T type a ‘reference’, and rather than owning the resource, it borrows ownership. A binding that borrows something does not deallocate the resource when it goes out of scope. This means that after the call to foo_2(), we can use our original bindings again.
    // References are immutable, just like bindings. This means that inside of foo_2(), the vectors can’t be changed at all

    println!("{}", answer);
    println!("{}", answer_2);
    println!("{:?}", v1_2);
    println!("{:?}", v2_2);

    // ---------------------------------------------

    // &mut references
    //There’s a second kind of reference: &mut T. A ‘mutable reference’ allows you to mutate (change, edit) the resource you’re borrowing.

    // Example

    let mut x: i32 = 5;

    {
        let y = &mut x;
        *y += 1;
        println!("Y: {}", y); // 6
    }

    println!("X: {}", x); // 6 as well

    // ---------------------------------------------

    // The rules of borrowing

    /*
    First, any borrow must last for a scope no greater than that of the owner. Second, you may have one or the other of these two kinds of borrows, but not both at the same time:

        - One or more references (&T) to a resource
        - Exactly one mutable reference (&mut T)
    */
}