//variables are immutable by default, but it can be overriden by the mut keyword

fn main() {
    let _immutable_binding = 1;
    let mut a_mutable_integer: i32 = 69;
    println!(
        "The current value for the variable is {}",
        a_mutable_integer
    );
    a_mutable_integer = 420;
    println!(
        "The value of the variable was changed to {}",
        a_mutable_integer
    );

    // _immutable_binding += 1;
    /* Uncommenting the above line will lead to this error
    error[E0384]: cannot assign twice to immutable variable `_immutable_binding`
      --> chapter04/src/bin/mutability.rs:16:5
       |
    4  |     let _immutable_binding = 1;
       |         ------------------ first assignment to `_immutable_binding`
    ...
    16 |     _immutable_binding += 1;
       |     ^^^^^^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
       |
    help: consider making this binding mutable
       |
    4  |     let mut _immutable_binding = 1;
       |         +++

    For more information about this error, try `rustc --explain E0384`.
    */
}
