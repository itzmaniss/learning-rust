fn main() {
    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // TODO ^ Try commenting out the `vec.push(elem)` line
    /*
    error[E0282]: type annotations needed for `Vec<_>`
     --> chapter05/src/bin/inference.rs:6:9
      |
    6 |     let mut vec = Vec::new();
      |         ^^^^^^^   ---------- type must be known at this point
      |
    help: consider giving `vec` an explicit type, where the type for type parameter `T` is specified
      |
    6 |     let mut vec: Vec<T> = Vec::new();
      |                ++++++++

    For more information about this error, try `rustc --explain E0282`.
     */

    println!("{:?}", vec);
}
