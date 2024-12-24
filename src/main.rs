fn main() {
    //let x = 5;
    //println!("value of x: {x}");

    //x = 6;
    //println!("value of x: {x}");
    // ^^^ compile error ^^^

    //let mut x = 5;
    //println!("value of x: {x}");

    //x = 6;
    //println!("value of x: {x}");

    //const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    //println!("This is constant: {THREE_HOURS_IN_SECONDS}")

    // ----------------- shadowing -----------------------
    let x = 5;
    println!("value of x: {x}");

    {
        let x = x * 2;
        println!("value of x: {x}");
    }

    println!("value of x: {x}");

    let x = 8;
    println!("value of x: {x}");

    let space = "   ";
    let space = space.len();
    println!("space: {space}")
}
