fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("{scores:?}");

    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
}
