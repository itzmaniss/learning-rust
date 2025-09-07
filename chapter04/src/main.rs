//Rust provides type safety via static typing. Variable bindings can be type annotated when declared.
// However, in most cases, the compiler will be able to infer the type of the variable from the context, heavily reducing the annotation burden.
// Values (like literals) can be bound to variables, using the let binding.

fn main() {
    let an_integer: i32 = 69;
    let a_bool: bool = true;
    let unit = ();

    //copy a variable
    let copied_integer = an_integer;

    println!("{} is an integer with type i32", copied_integer);
    println!("{} is a boolean", a_bool);
    println!("This is what a unit variable looks like {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser
}
