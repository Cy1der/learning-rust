fn main() {
    // A ‘vector’ is a dynamic or ‘growable’ array, implemented as the standard library type Vec<T>. The T means that we can have vectors of any type. Vectors always allocate their data on the heap.

    let v = vec![1, 2, 3]; // Type is Vec<i32>
    let v2 = vec![1; 10]; // 10 ones

    // Accessing values (zero indexed!)

    println!("{:?} index 2: {}", v, v[1]);
    println!("{:?} index 6: {}", v2, v2[5]);

    // Iterating

    let v3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in &v3 {
        println!("v3 reference: {}", i);
    }
}