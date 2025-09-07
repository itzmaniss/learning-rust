use crate::List::*;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, val: i32) -> List {
        Cons(val, Box::new(self))
    }
    // `self` has to be matched, because the behavior of this method
    // depends on the variant of `self`
    // `self` has type `&List`, and `*self` has type `List`, matching on a
    // concrete type `T` is preferred over a match on a reference `&T`
    // after Rust 2018 you can use self here and tail (with no ref) below as well,
    // rust will infer &s and ref tail.
    // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    // Can't take ownership of the tail, because `self` is borrowed;
    // instead take a reference to the tail
    // And it'a a non-tail recurssive call which may cause stack overflow for long lists.

    // BRO WHY IS IT SO LONG DA HELLLLLLL. ISSOK SO BASICALLY YOU DONT OWN SELF, COS IF YOU DO YOU WILL DROP THE
    // SELF SO YOU GET THE REFERENCE RIGHT, BUT NOW SINCE YOU WANT THE VALUE YOU NEED TO DE REFERENCE IT
    // SO YOU USE * IF NOT YOU JUST GOT THE REFERENCE TO MATCH WITH AND NOT THE VALUE LMAO YE BUT NOW BASICALLY,
    // THE COMPILER IS SO SMART MATCH ALSO DONT NEED THIS DE REFERENCING SO LIKEEEEEEEEEEEE

    //to find length we just gott use a match case to recursively add one where if self is nil we return 0
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    //Adapted the method to remove trailing commas, instead of how the book left it
    fn stringify(&self) -> String {
        match *self {
            Cons(val, ref tail) => match **tail {
                Nil => format!("{}", val),
                _ => format!("{}, {}", val, tail.stringify()),
            },
            Nil => format!(""),
        }
    }
    // I wanted to take it a step further and try to implement the append function for this linked list as well.
    // Whar do i gotta do for append?
    // 1) Traverse to the end of the list using matching
    // 2) once i match with Nil, i cahnge the nil to a new List node
    // -- I need to stop at the node before the last...
    fn append_in_place(&mut self, val: i32) {
        match *self {
            //  WE HAVE TO DOUBLE DE-REF THE TAIL. WHY?
            // TAIL = REF OF BOX(LIST) THUS, *TAIL = BOX(LIST) THUS **TAIL = LIST
            Cons(_, ref mut tail) => match **tail {
                Nil => **tail = Cons(val, Box::new(Nil)),
                Cons(_, _) => tail.append_in_place(val),
            },
            //GOT STUCK AT BASE CASE,
            // COMPLETELY FORGOT THAT LIST CAN BE EMPTY AND I NEED TO JUST MAKE THE LIST IF IT IS EMPTY
            // LOL!!!
            Nil => *self = Cons(val, Box::new(Nil)),
        }
    }
    //how can i append to the list via rebuilding the list?
    //
    fn append(self, val: i32) -> List {
        match self {
            Cons(current_val, next) => Cons(current_val, Box::new(next.append(val))),
            Nil => Cons(val, Box::new(Nil)),
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
    list.append_in_place(42);
    println!("After appending in place: ");
    println!("Linked list now has length: {}", list.len());
    println!("New List: {}", list.stringify());
    let list = list.append(69);
    println!("After appedning via rebuilding: ");
    println!("Linked list now has length: {}", list.len());
    println!("New list is: {}", list.stringify())
}
