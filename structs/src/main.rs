#![allow(unused, dead_code)]

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // Note: convenience syntax here
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User1's email is {}", user1.email);

    user1.email = String::from("foo@bar.com");
    println!("User1's email is now {}", user1.email);

    // Struct update syntax
    let user2 = User {
        email: String::from("bar@baz.com"),
        username: String::from("another"),
        ..user1
    };
    println!("User2's email is {}", user2.email);

    // Tuple structs
    struct Colour(i32, i32, i32);
    let black = Colour(0, 0, 0);
    println!("black is ({},{},{})", black.0, black.1, black.2);

    // Unit-like structs
    struct Foo();

    // Ownership of struct data
    // In above def, User struct owns all its data. Could instead use references,
    // but this needs to involve concept of lifetimes.
    // This won't work currently:
    // struct User2 {
    //     username: &str,
    // }
    // let user = User2 {
    //     username: "user",  // error[E0106]: missing lifetime specifier
    // };
}
