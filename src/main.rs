#[derive(Debug)]
struct Rectangle {
    width: u32,
    hight: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        hight: 50,
    };

    let rect1 = dbg!(rect1); // dbg! takes ownership, rect1 is moved
    println!("{}", rect1.width)
}
