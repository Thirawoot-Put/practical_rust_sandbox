fn main() {
    let mut v = vec![1, 2, 3, 4];

    v.push(6);
    for e in &v {
        println!("{e}");
    }

    println!("---------------");
    let a = v.pop();
    for e in &v {
        println!("{e}");
    }

    println!("---------------");
    match a {
        Some(val) => println!("{val}"),
        None => println!("a is none"),
    }
}
