fn main() {
    let mut s = String::new();

    let data = "initial contents";

    s = data.to_string();
    println!("{s}");

    // the method also works on a literal directly:
    s = "initial contents".to_string();
    println!("{s}");

    let s = String::from("hello contents");
    println!("{s}");
}
