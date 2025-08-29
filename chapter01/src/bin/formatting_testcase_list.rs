// Implementing fmt::Display for a structure where the elements must each be handled sequentially is tricky. 
// The problem is that each write! generates a fmt::Result. 
// Proper handling of this requires dealing with all the results. 
// Rust provides the ? operator for exactly this purpose.

// Try `write!` to see if it errors. If it errors, return
// the error. Otherwise continue.
// write!(f, "{}", value)?;

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        //extract the value via tuple indexing and create a reference to vec
        let vec = &self.0;

        write!(f, "[")?;

        //iterate through the vector and write each element
        // Iterate over `v` in `vec` while enumerating the iteration
        // index in `index`.
        for(index, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

struct NumberedList(Vec<i32>);

impl fmt::Display for NumberedList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (index, v) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; }

            write!(f, "{}: {}", index, v)?;
        }
        write!(f, "]")
    }
}

fn main(){
    let list = List(vec![1, 2, 3]);

    println!("{}", list);

    let numbered_list = NumberedList(vec!(1, 2, 3));

    println!("{}", numbered_list);
}