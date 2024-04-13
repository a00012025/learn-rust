#![allow(dead_code)]

use rand::Rng;

fn noisy_unused_function() {}

fn create_box() {
    // Allocate an integer on the heap
    let _box1: Box<i32> = Box::new(3i32);
    // `_box1` is destroyed here, and memory gets freed
}

#[derive(Debug)]
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
    age2: u8,
}

#[derive(Debug, Clone)]
struct Book {
    author: String,
    title: String,
    year: u32,
}

fn main() {
    println!("Hello, world!");
    // get random int from 1 to 50
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..51);
    println!("The secret number is: {}", secret_number);

    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);
    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }
    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }
    // `_box2` is destroyed here, and memory gets freed

    let _x: ToDrop = ToDrop;
    println!("Made a ToDrop!");

    let _y = _x;
    println!("Made a move!");
    // println!("{:?}", _x);

    let mut box1 = Box::new(2u32);
    *box1 = 23u32;
    println!("box1: {}", box1);
    let box2 = box1;
    println!("box2: {}", box2);
    // println!("box1: {}", box1);

    let p = Person {
        name: String::from("Alice"),
        age: Box::new(20),
        age2: 30,
    };
    println!("{:?}", p);
    let Person {
        name,
        ref age,
        age2,
    } = p;
    println!("name: {}, age: {}, age2: {}", name, age, age2);
    println!("{:?}", p.age);
    println!("{:?}", p.age2);
    // println!("{:?}", p.name);
    // println!("{:?}", p);

    let boxed_i32 = Box::new(6i32);
    let boxed_i32_ref: &i32 = &boxed_i32;
    println!("boxed_i32_ref: {}", boxed_i32_ref);
    println!("boxed_i32: {}", boxed_i32);

    let mut book1 = Book {
        author: String::from("John Doe"),
        title: String::from("Rust Programming"),
        year: 2021,
    };
    let book1_1 = &mut book1;
    // let book1_2 = &book1;
    println!("{:?}", book1_1);
    let ref book1_2 = book1;
    let ref book1_3 = book1;
    println!("{:?}", book1_2);
    println!("{:?}", book1_3);
}
