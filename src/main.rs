fn main() {
    //let mut s = String::from("hello");

    //let r1 = &mut s;
    //let r2 = &mut s;
    //
    //println!("{},{}", r1, r2); // cannot 'borrow' "s" more than once at a time

    //let r1 = &mut s;
    //println!("r1: {}", r1);
    //let r2 = &mut s;
    //
    //println!("r2: {}", r2); // work because not 'borrow' more than once at a time
    //
    //{
    //    let r3 = &s;
    //    println!("r3: {}", r3);
    //}
    //let r4 = &s;
    //println!("r4: {}", r4);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    //println!("{r1} and {r2}"); compile error, Rust will determin it's ่
}
