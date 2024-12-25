fn main() {
    print_labeled_measurement(5, 'h');

    //let x = (let y = 6); // error: let y = 6 is statement

    let x = {
        let y = 6 + 2;
        y
    };
    println!("value of x: {x}");

    let plus = plus_one(x);
    println!("plus value: {plus}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
