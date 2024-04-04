use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Pair1 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Pair2 {
    x: f32,
    y: f32,
}

impl From<Pair1> for Pair2 {
    fn from(pair: Pair1) -> Self {
        Pair2 {
            x: pair.x as f32,
            y: pair.y as f32,
        }
    }
}

impl From<Pair2> for Pair1 {
    fn from(pair: Pair2) -> Self {
        Pair1 {
            x: pair.x as i32,
            y: pair.y as i32,
        }
    }
}

use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

#[derive(Debug)]
struct MyNumber(i32);

impl FromStr for MyNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i32>() {
            Ok(n) => Ok(MyNumber(n)),
            Err(_) => Err(()),
        }
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    let decimal = 6522.4321_f32;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!("1000 as a u8 is : {}", 1000i32 as u8);

    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }

    let mut vec = Vec::<u8>::new();
    vec.push(5u8);
    println!("{:?}", vec);

    let my_str = "abc";
    let my_string = String::from(my_str);
    println!("{} , {}", my_str, my_string);

    // from pair1 to pair2
    let pair1 = Pair1 { x: 1, y: 2 };
    let pair2: Pair2 = pair1.into();
    println!("{:?}", pair2);
    println!("OK {:?}", Pair2::from(pair1));

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let n2 = MyNumber(5);
    let n3 = MyNumber::from_str("10").unwrap();
    let n4: MyNumber = "13".parse().unwrap();
    let n5 = "18".parse::<MyNumber>().unwrap();
    println!("{:?} {:?} {:?} {:?}", n2, n3, n4, n5);
}
