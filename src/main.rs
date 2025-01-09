use crate::garden::vegetables::Asparagus;
// use crate::garden::vegetables::Brocoli; // error: Brocoli is private (not use "pub" inline it)
use crate::garden::vegetables::eat_brocoli;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!!!");

    eat_brocoli();
}
