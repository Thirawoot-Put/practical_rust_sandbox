#[derive(Debug)]
pub struct Asparagus {}

#[derive(Debug)]
struct Brocoli {}

pub fn eat_brocoli() {
    let plant = Brocoli {};
    println!("Inside vegetables garden, I'm eating {plant:?}.")
}
