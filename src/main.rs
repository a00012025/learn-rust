use std::fmt;

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(DebugPrintable);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = &self.0;
        write!(f, "[")?;
        for (i, v) in v.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    println!("{:>5} days", 21);

    #[allow(dead_code)]
    struct Structure(i32);
    // println!("This struct `{}` won't print...", Structure(3));
    println!("This struct `{:?}` will print...", DebugPrintable(3));
    println!("This struct `{:?}` will print...", DebugPrintable(3).0);
    // println!("This struct `{}` won't print...", DebugPrintable(3));
    println!("This struct `{:?}` will print...", Deep(DebugPrintable(30)));
    println!("This struct `{:?}` will print...", Deep(DebugPrintable(30)).0.0);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // print floating number
    println!("{number:.3}", number=3.1415926);

    // print Person
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);
    println!("{:#?}", peter);

    println!("Number: 0x{:0>2X}{:0>2X}", 123, 12)
}

