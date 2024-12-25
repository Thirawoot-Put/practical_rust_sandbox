//use std::mem;
use std::io;

fn main() {
    // ************* Scalar types *****************

    // --------------- Integer --------------------
    //let a: u8 = 255;
    //println!(
    //    "a is unsigned 8 bits: value == {a}, size == {}",
    //    mem::size_of_val(&a)
    //);
    //
    //let a = 300;
    //println!(
    //    "a is overflow form it's initial size: value == {a}, size == {}",
    //    mem::size_of_val(&a)
    //);
    //
    //let b: i8 = -128;
    //println!("b is signed 8 bits: {b}");
    //
    //let b: i8 = 127;
    //println!("b was shadowed still in signed: {b}");
    //
    //let x = 1_000;
    //println!("x with underscore for seperator: {x}");
    //
    //let hx = 0xff;
    //println!("hx is in hex form for 0xff: {hx}");
    //
    //let oc = 0o77;
    //println!("oc is in octal form for 0o77: {oc}");
    //
    //let bi = 0b1111_0000;
    //println!("bi is in binary form for 0b1111_0000: {bi}");
    //
    //let byt = b'A';
    //println!("byt is in byte form (u8 only) for b'A': {byt}");

    // --------------- Floating-Point --------------------
    //let a = 2.1; // f64
    //let b: f32 = 3.001; // f32
    //
    //println!("a: {a}, and b: {b}")

    // --------------- Numeric operations --------------------
    //// addition
    //let sum = 5 + 10;
    //
    //// subtraction
    //let difference = 95.5 - 4.3;
    //
    //// multiplication
    //let product = 4 * 30;
    //
    //// division
    //let quotient = 56.7 / 32.2;
    //let truncated = -5 / 3; // Results in -1
    //
    //// remainder
    //let remainder = 43 % 5;

    // --------------- Boolean --------------------
    //let t = true;
    //
    //let f: bool = false; // with explicit type annotation

    // --------------- Character --------------------
    //let c = 'z';
    //let z: char = 'ℤ'; // with explicit type annotation
    //let heart_eyed_cat = '😻';

    // *********** Compound types ***************
    // ------------ Tuple ----------------
    //let tup: (i32, f64, u8) = (500, 6.4, 1);

    //let tup = (500, true, 1);
    //let (x, y, z) = tup;
    //
    //println!("The value of y is: {y}");
    //
    //// => destructuring
    //let x: (i32, f64, u8) = (500, 6.4, 1);
    //
    //let five_hundred = x.0;
    //
    //let six_point_four = x.1;
    //
    //let one = x.2;

    // ------------ Array ----------------
    //let test_err = ['a', 2]; // error defferent type in array
    //
    //let a: [i32; 5] = [1, 2, 3, 4, 5];
    //let b = [3; 5]; // [3, 3, 3, 3, 3,]
    //const LEN: usize = 5;
    //let arr: [i32; LEN] = [1, 1, 1, 1, 1];
    //let a = [1, 2, 3, 4, 5];
    //
    //let first = a[0];
    //let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
