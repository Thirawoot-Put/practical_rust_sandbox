fn main() {
    let v = vec![1, 2, 3, 4];

    let third: &i32 = &v[2];
    println!("Third index of v is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("Third index of v is {value}"),
        None => println!("There is no third element"),
    }

    let fifth = v.get(4); // return Option<T>
    match fifth {
        Some(value) => println!("Third index of v is {value}"),
        None => println!("There is no fifth element"),
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
