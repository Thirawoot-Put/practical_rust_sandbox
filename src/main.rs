fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        //.split_whitespace() return iterator that return string slices which is
        //sub-slice of original string
        println!("{word}");
        let count = map.entry(word).or_insert(0);
        *count += 1; // use return form or_insert(reference to that value) to increment it by
                     // dereference with *
    } // mutable reference end here

    println!("{map:?}");
}
