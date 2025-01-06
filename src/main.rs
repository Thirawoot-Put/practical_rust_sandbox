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

    dbg!(&rect1);
    println!("{}", rect1.width)
}
