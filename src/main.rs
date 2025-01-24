use std::collections::HashMap;

fn main() {
    let mut v = [1, 24, 3, 2, 7, 8, 2];

    let (_lesser, median, _greater) = v.select_nth_unstable_by(3, |a, b| b.cmp(a));
    println!("median is {median}");

    let mut h = HashMap::new();
    for n in v {
        let count = h.entry(n).or_insert(0);
        *count += 1
    }

    let mut count_arr = Vec::new();
    for (_n, count) in h {
        count_arr.push(count);
    }

    count_arr.sort();
    let most = count_arr[count_arr.len() - 1];
    println!("{most}")
}
