#[allow(unused_variables)]
fn main() {
    let s1 = String::from("foo");
    let s2 = s1;

    // This will result in "error[E0382]: borrow of moved value: `s1`",
    // and is how rust avoids a double-free.
    // println!("{}", s1);

    // Instead, we can explicitly clone (possibly an expensive op). This is not
    // needed for primitive types. This can also be achieved for any type by
    // implementing Copy (assuming no parts of the type implement Drop).
    let s1 = String::from("foo");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership & functions

    let s = String::from("foo");
    takes_ownership(s);
    // println!("{}", s); => error[E0382]: borrow of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);

        // s1 & s2 go out of scope and are dropped
        // s2 goes out of scope, but was moved, so nothing happens
    }

    // Returning a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
