struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1 email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // user1.username is moved here
    };
    //println!("user1 username: {}", user1.username); // compile error: user1.username was moved
    println!("user2 email: {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    //User {
    //    active: true,
    //    username: username,
    //    email: email,
    //    sign_in_count: 1,
    //};

    User {
        active: true,
        username, // short hand
        email,
        sign_in_count: 1,
    }
}
