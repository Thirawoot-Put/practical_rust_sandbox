fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("value of six == {:?}", six);
    println!("value of none == {:?}", none);
}

fn plus_one(n: Option<i32>) -> Option<i32> {
    match n {
        None => None,
        Some(v) => Some(v + 1),
    }
}
