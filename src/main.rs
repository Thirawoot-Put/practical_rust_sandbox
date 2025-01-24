fn main() {
    let mut vec = [1, 24, 3, 2, 7, 8, 2];

    let (_lesser, median, _greater) = vec.select_nth_unstable_by(3, |a, b| b.cmp(a));
    println!("median is {median}");
}
