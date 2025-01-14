fn main() {
    let mut s = String::from("foo");
    let r_s = &s;
    println!("1---> s: {r_s:p} capacity {}", s.capacity());

    let s2 = String::from("bar hello word baba");
    let r_s2 = &s2;

    s.push_str(&s2);
    let r_s = &s;

    println!("2---> s: {r_s:p} capacity {}", s.capacity());
    println!("---> s2: {r_s2:p}");
}
