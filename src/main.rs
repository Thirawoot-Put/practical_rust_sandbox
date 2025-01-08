#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let quarter = Coin::Quarter(UsState::Alaska);
    value_in_cents(quarter);

    let penny = Coin::Penny;
    value_in_cents(penny);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // bind state => UsState::Alaska
            println!("State quarter from {state:?}!");
            25
        }
    }
}
