fn main() {
    //loop {
    //    println!("again!");
    //}

    //let mut counter = 0;
    //
    //let result = loop {
    //    counter += 1;
    //
    //    if counter == 10 {
    //        break counter * 2; // return value out of loop by put return value after break
    //                           // return counter * 2 // this is return value and out of function
    //    }
    //};
    //
    //println!("The result is {result}");

    //let mut count = 0;
    //'counting_up: loop {
    //    println!("count = {count}");
    //    let mut remaining = 10;
    //
    //    loop {
    //        println!("remaining = {remaining}");
    //        if remaining == 9 {
    //            break;
    //        }
    //        if count == 2 {
    //            break 'counting_up;
    //        }
    //        remaining -= 1;
    //    }
    //
    //    count += 1;
    //}
    //println!("End count = {count}");

    //let mut number = 3;
    //
    //// do it when condition is true and call break when condition is false
    //while number != 0 {
    //    println!("{number}!");
    //
    //    number -= 1;
    //}
    //
    //println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // (1..4): Range standard library, == 1 - 4 and for loop do it until before last one (4)
    // => do it form 1 to 3
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
