fn main() {

    //formating strings
    println!("I am {} years old", 21);

    //formatting with indexes
    println!("{0}, This is {1}, {1}, This is {0}", "Ric", "Swampy");

    //formating with key value pairs
    println!("Hi, I am {name}, I am {age} years old as of {year}", name="itzmaniss", age="21", year="2025");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10         :{}", 69420);
    println!("Base 2  (binary):{:b}", 69420);
    println!("Base 8  (octa)  :{:o}", 69420);
    println!("Base 16 (hexa)  :{:x}", 69420);

    // You can right-justify text with a specified width. This will
    // output "   69". (3 white spaces and a "69", for a total width of 5.)
    println!("{number:>5}", number=69);

    //can pad with other characters as well
    println!("{number:0>5}", number = 69);
    //can pad left by switching the sign
    println!("{number:0<5}", number = 69);

    //can make the padding a variable as well by appending a $ 
    println!("{number:0<width$}", number=69, width = 5);

    /* //rust makes sure there are correct number of inputs
    println!("My name is {0}, {1} {0}", "Bond") --> 

    error: invalid reference to positional argument 1 (there is 1 argument)
    --> formattedPrint.rs:32:32
    |
    32 |     println!("My name is {0}, {1} {0}", "Bond")
    |                                ^
    |
    = note: positional arguments are zero-based

    error: aborting due to 1 previous error */
    //The one below works as expected
    println!("My name is {0}, {1} {0}", "Bond", "James");

    /*
    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.
 
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line
    //error[E0277]: `Structure` doesn't implement `std::fmt::Display`
    */


    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".

    //👉 Instead of passing variables as extra arguments to println!, 
    // you can write {var} directly in the string, and Rust will format it for you.

    let number: f64 = 69.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f32 = 3.141592;
    println!("pi is {pi:.3} to 3 decimal places");
}