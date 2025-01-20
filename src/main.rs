fn main() {
    let greeting: &str = "Hello, world!";
    let greeting2: &str = "Hello, world!"; // Same string literal as `greeting`
    let greeting3: &str = "Hello, banana"; // Different string literal

    // Print memory addresses of all three
    println!("Address of greeting: {:p}", greeting);
    println!("Address of greeting2: {:p}", greeting2);
    println!("Address of greeting3: {:p}", greeting3);

    // Compare addresses
    println!(
        "Do greeting and greeting2 point to the same address? {}",
        std::ptr::eq(greeting, greeting2)
    );
    println!(
        "Do greeting3 point to the same address as greeting or greeting2? {}",
        std::ptr::eq(greeting3, greeting)
    );
    println!(
        "Do greeting3 point to the same address as greeting2? {}",
        std::ptr::eq(greeting3, greeting2)
    );

    let greeting: &str = "Hello, world!"; // String literal, a type of string slice
    let dynamic_string = String::from("Rust is awesome!"); // Heap-allocated string
    let slice = &dynamic_string[0..4]; // String slice of a heap-allocated string
    println!("{}", slice); // Output: Rust
}
