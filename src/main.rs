fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The len of word {s1} is {len}")
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
