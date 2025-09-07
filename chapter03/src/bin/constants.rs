// const: An unchangeable value (the common case).

// static: A possibly mutable variable with 'static lifetime.
// The static lifetime is inferred and does not have to be specified.
// Accessing or modifying a mutable static variable is unsafe.
//
// Static is meant to be used for shared states or data that cant be determined at compile time --> I also dont fully understand it yet

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(val: i32) -> bool {
    val > THRESHOLD
}

fn main() {
    let val: i32 = 69;
    println!("The language is {}.", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", val, if is_big(val) { "Big" } else { "Small" });

    //error! cannot modify a const;
    // THRESHOLD = 420;
    // COmment out the line to fix/un comment to see error sample
    /*
    * error[E0070]: invalid left-hand side of assignment
      --> chapter03/src/bin/constants.rs:21:15
       |
    21 |     THRESHOLD = 420;
       |     --------- ^
       |     |
       |     cannot assign to this expression

    For more information about this error, try `rustc --explain E0070`.
    */
}
