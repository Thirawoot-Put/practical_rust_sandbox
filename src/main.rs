fn main() {
    let a: u8 = 255;
    println!("a is unsigned 8 bits: {a}");

    let b: i8 = -128;
    println!("b is signed 8 bits: {b}");

    let b: i8 = 127;
    println!("b was shadowed still in signed: {b}");

    let x = 1_000;
    println!("x with underscore for seperator: {x}");

    let hx = 0xff;
    println!("hx is in hex form for 0xff: {hx}");

    let oc = 0o77;
    println!("oc is in octal form for 0o77: {oc}");

    let bi = 0b1111_0000;
    println!("bi is in binary form for 0b1111_0000: {bi}");

    let byt = b'A';
    println!("byt is in byte form (u8 only) for b'A': {byt}");
}
