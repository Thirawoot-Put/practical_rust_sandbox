fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The len of word {s1} is {len}");

    //let s2 = change(&s1);

    let mut s2 = String::from("hello");
    println!("The value of s2 is {s2}");

    change(&mut s2);
    println!("The value of s2 is {s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//fn change(some_string: &String) {
//    some_string.push_str(", wolrd");
//} // compile error cannot change variable that immutable by default

fn change(some_string: &mut String) {
    some_string.push_str(", wolrd");
}
