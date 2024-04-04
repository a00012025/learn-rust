use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v: &Vec<i32> = &self.0;
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

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} + {:.1}i", self.real, self.imag)
    }
}

fn main() {
    let s = Structure(3);
    println!("Display: {}", s);
    println!("Debug: {:?}", s);

    let m = MinMax(0, 14);
    println!("Debug print: {:?}", m);
    println!("Display print: {}", m);

    let c = Complex {
        real: 3.33,
        imag: 7.22,
    };
    println!("Display: {}", c);
    println!("Debug: {:?}", c);

    let v = List(vec![12, 23, 31]);
    println!("List: {}", v);
}
