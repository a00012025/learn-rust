#[allow(dead_code)]
use std::fmt;
use std::mem;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    (x2 - x1) * (y2 - y1)
}

fn main() {
    println!("Hello, world!");

    let mut inferred_type = 12i64; // Type i64 is inferred from another line.
                                   // inferred_type = 4294967296i64;
    inferred_type = 32;
    println!("inferred_type: {}", inferred_type);

    let a = 1i32;
    let b = 2i32;
    println!("1 - 2 = {}", a - b);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First element of the array: {}", xs[0]);
    println!("Number of elements in array: {}", xs.len());
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    for i in 0..xs.len() + 1 {
        let elem = xs.get(i);
        match elem {
            Some(value) => println!("Value of element {} is {}", i, value),
            None => println!("No value found for element {}", i),
        }
    }

    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let tup1 = (1, 0.1);
    let (integer1, decimal1) = tup1;
    println!("tup1 contains {:?} and {:?}", integer1, decimal1);
}
