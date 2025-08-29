// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

//to print debug statements we use {:?} --> this works similar to {} format
fn main() {

    println!("There are {:?} months in a year", 12);

     //when you see the print statement you can see the quotation marks cos debug!!
    println!("{1:?} {0:?} is the actor in {movie:?}", "Reynolds", "Ryan", movie="Deadpool");

    //Unlike the formatted prints example, now the struct here is printable!
    println!("Now {:?} will print", Structure(69));
    println!("{:?} is a structure in a structure and it prints too!", Deep(Structure(420)));

    //println expects a string or data type at all times? so cannot just pass a function like in python, need format it
    // i.e. println!("{}", func());
    second();
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn second() {
    let name = "itzmaniss";
    let age = 21;
    // use curly braces when using struct
    let me = Person{name, age};

    // # lets us do inbuilt pretty print --> {:#?}
    println!("{:#?}", me);
}

