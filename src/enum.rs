use std::fmt;

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

#[derive(Debug)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

use crate::List::*;

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

static LANGUAGE: &str = "Rust";
const LANGUAGE2: &str = "Rust";
const THRESHOLD: i32 = 10;

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    println!("pressed: {:?}", pressed);
    println!("pasted: {:?}", pasted);
    println!("click: {:?}", click);
    println!("load: {:?}", load);
    println!("unload: {:?}", unload);

    match pressed {
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
        WebEvent::PageLoad => println!("page loaded."),
        WebEvent::PageUnload => println!("page unloaded."),
    }
    println!("enum to i32: {}", Work::Civilian as i32);

    use Status::{Poor, Rich};
    use Work::*;
    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;
    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    println!("This is {}", LANGUAGE);
    println!("This is {}", LANGUAGE2);
    println!("The threshold is {}", THRESHOLD);
}
