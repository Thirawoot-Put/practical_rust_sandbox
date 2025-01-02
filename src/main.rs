fn main() {
    // **** String type ****
    //let mut s = String::from("hello");
    //
    //s.push_str(", world!"); // push_str() appends a literal to a String
    //
    //println!("{s}"); // This will print `hello, world!`

    //let s1 = String::from("hello");
    //let s2 = {
    //    println!("{s1}");
    //    s1
    //};
    //
    //println!("{s2}, world!");
    //println!("{s1}, world!"); // compile error: s1 is out of scope; Rust call dropp function;
    //Rust invalidates s1; cannot use it any more

    //let mut s = String::from("hello");
    //s = String::from("ahoy");
    //
    //println!("{s}, world!");

    let mut s = String::from("hello");
    println!("before: {s}, world!");

    s = String::from("ahoy");

    println!("after: {s}, world!");
}
