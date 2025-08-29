//Import fmt via use statement to make it available
use std::fmt;

struct Structure(i32);
// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
// we will use the impl keyword to implement
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`

        write!(f, "Your structure contains the value {}", self.0)
    }
}

/*
There is no ideal style for all types and the std library doesn't presume to dictate one. 
fmt::Display is not implemented for Vec<T> or for any other generic containers. 
fmt::Debug must then be used for these generic cases.
or a fmt::Display must be implemented by the user
*/

#[derive(Debug)]
struct MinMax(i64, i64);

//implement a display format for MinMax

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

//make a struct with key value pairs for comparison
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "x: {}, y: {}", self.x, self.y);
    }
}

// After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. 
// When printed in the same way, the output should be:
// Display: 3.3 + 7.2i
// Debug: Complex { real: 3.3, imag: 7.2 }

#[derive(Debug)]
struct ComplexNum {
    real: f32,
    img: f32
}

impl fmt::Display for ComplexNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real} + {img}i", real=self.real, img=self.img)
    }
}



fn main() {

    let x = Structure(69);
    println!("{}\n", x);
    
    let y = MinMax(69, 420);
    println!("Debug: {:#?}", y);
    println!("Display: {}\n", y);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}\n",
             small = small_range,
             big = big_range);

    let z = Point2D{x:3.3, y:7.2};

    println!("Compare points: ");
    println!("Display: {}", z);
    println!("Debug: {:#?}\n", z);

    let complex_number = ComplexNum{real:3.3, img:7.2};
    println!("Debug: {:?}", complex_number);
    println!("Display: {}", complex_number);


}

// Error. Both `Debug` and `Display` were implemented, but `{:b}`
// requires `fmt::Binary` to be implemented. This will not work.
// println!("What does Point2D look like in binary: {:b}?", point);
// --> error[E0277]: the trait bound `Point2D: Binary` is not satisfied
// So, fmt::Display has been implemented but fmt::Binary has not, and therefore cannot be used. 
// std::fmt has many such traits and each requires its own implementation. 
// This is detailed further in std::fmt.