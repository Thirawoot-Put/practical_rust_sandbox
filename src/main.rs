struct Rectangle {
    width: u32,
    hight: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        hight: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1) // ownership: moved occur here because not pass reference, this pass whole
                    // instance of struct
    );

    println!(
        "The width of the rectangle is {}, area is {}",
        two_multiply_width(&rect1),
        area(rect1)
    ); // not error because width is simple scalar

    println!("Current width {}", rect1.width)
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.hight
}

fn two_multiply_width(rectangle: &Rectangle) -> u32 {
    rectangle.width * 2
}
