#[allow(unused_variables, unused_mut)]
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    // Problem: word will still have value 5 here

    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];

    let mut s = String::from("hello world");
    let word = first_word_slice(&s);
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();
    println!("First word is {}", word);

    // Note: string literals are slices pointing to the string in the binary
    let s = "Hello, world!";  // This is a &str

    // See note below
    let s = String::from("hello world");
    let word = first_word_slice(&s[..]);
    let s = "hello world";
    let word = first_word_slice(&s[..]);
    // String literals are slices already
    let word = first_word_slice(&s);

    // Can also have slices of other types
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

// Note: writing the parameter as '&str' instead of '&String' allows us to support both types
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
