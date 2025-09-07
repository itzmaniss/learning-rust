// It is possible to declare variable bindings first and initialize them later,
// but all variable bindings must be initialized before they are used: the compiler forbids use of uninitialized variable bindings,
// as it would lead to undefined behavior.

fn main() {
    //declare a variable binding
    let a_binding;

    {
        let x = 2;

        //initialise the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    //this will give an error cos not initialised;
    // println!("{}", another_binding);
    /*
    error[E0381]: used binding `another_binding` is possibly-uninitialized
      --> chapter04/src/bin/declare_first.rs:21:20
       |
    18 |     let another_binding;
       |         --------------- binding declared here but left uninitialized
    ...
    21 |     println!("{}", another_binding);
       |                    ^^^^^^^^^^^^^^^ `another_binding` used here but it is possibly-uninitialized
       |
       = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

    For more information about this error, try `rustc --explain E0381`.
     */

    another_binding = 1;
    println!("another binding: {}", another_binding)
}
