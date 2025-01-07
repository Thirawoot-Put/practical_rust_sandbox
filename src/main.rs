//inside standard library: Option
//enum Option<T> {
//    None,
//    Some(T),
//}

fn main() {
    let some_number = Some(5); // Rust infer some_number has type Option<i32> **i32 because it's
                               // default
    let some_char = Some('e'); // Rust infer some_char has type Option<char>

    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}", some_number, some_char, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y; // compile error: they are different type (Option<i8> != i8)
}
