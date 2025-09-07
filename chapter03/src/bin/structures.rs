/*
There are three types of structures ("structs") that can be created using the struct keyword:
- Tuple structs, which are, basically, named tuples.
- The classic C structs
- Unit structs, which are field-less, are useful for generics.
*/

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

//making meself some structs

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

//a unit struct
struct Unit;

//a tuple struct
struct Pair(i32, f32);

//a struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    //initiate 2 points
    let point: Point = Point { x: 6.9, y: 4.2 };
    let another_point: Point = Point { x: 69f32, y: 0.42 };

    //access fields of a point struct:
    println!("Point coordinates are: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure a struct using the let binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        //struct initialisation is a expression too!
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    //initiate a unit strcuture
    let _unit = Unit;

    //initiate a tuple struct
    let pair = Pair(1, 0.1);

    //access the field of a tuple struct
    println!("The pair contains {:?}, {:?}", pair.0, pair.1);

    //destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!(
        "The destructured pair contains {:?}, {:?}",
        integer, decimal
    )
}
