// Variable bindings have a scope, and are constrained to live in a block.
// A block is a collection of statements enclosed by braces {}.

fn main() {
    //this binding lives in the main functions
    let long_lived_binding = 1;
    let shadow_binding = 1;

    //this is a block
    {
        //this binding only live inside the block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    println!("Outer long: {}", long_lived_binding);

    //this println will casue an error as the binding is not inside the scope
    // println!("Outer short: {}", short_lived_bindng);
    /*
    error[E0425]: cannot find value `short_lived_bindng` in this scope
      --> chapter04/src/bin/scope_and_shadowing.rs:18:33
       |
    18 |     println!("Outer short: {}", short_lived_bindng);
       |                                 ^^^^^^^^^^^^^^^^^^ help: a local variable with a similar name exists: `long_lived_binding`

    For more information about this error, try `rustc --explain E0425`.
     */

    println!("\nNow for shadow binding\n");
    {
        //for shadow binding
        println!("Inner Before Shadowing: {}", shadow_binding);
        let shadow_binding = "HI I AM INSIDE A BLOCK!!";
        println!("Inner after shadowing: {}", shadow_binding);
    }
    println!("outside after inner shadowing: {}", shadow_binding);
    let shadow_binding = 2;
    println!("Outside after shadowing: {}", shadow_binding);
}
