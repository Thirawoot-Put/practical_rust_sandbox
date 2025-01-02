fn main() {
    let mut s = String::from("hello");

    //let r1 = &mut s;
    //let r2 = &mut s;
    //
    //println!("{},{}", r1, r2); // cannot 'borrow' "s" more than once at a time

    let r1 = &mut s;
    println!("r1: {}", r1);
    let r2 = &mut s;

    println!("r2: {}", r2); // work because not 'borrow' more than once at a time

    {
        let r3 = &s;
        println!("r3: {}", r3);
    }
    let r4 = &s;
    println!("r4: {}", r4);
}
