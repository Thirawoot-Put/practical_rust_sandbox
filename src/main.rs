#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nikel,
    Dime,
    Quarter(UsState),
}

fn main() {
    count_none_quarter_coin(Coin::Penny);
    count_none_quarter_coin(Coin::Quarter(UsState::Alaska));
}

fn count_none_quarter_coin(coin: Coin) {
    let mut count = 0;

    //match coin {
    //    Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    //    _ => {
    //        count += 1;
    //        println!("count: {count}");
    //    }
    //}

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
        println!("count: {count}");
    }
}
