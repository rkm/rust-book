#[allow(unused_variables)]
fn main() {

    // Summary:
    // At any given time, you can have either one mutable reference or any
    // number of immutable references.
    // References must always be valid.

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    // error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
    // change(&s2);
    change_mut(&mut s2);

    let r1 = &mut s2;
    let r2 = &mut s2;
    // error[E0499]: cannot borrow `s2` as mutable more than once at a time
    //println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    let r1 = &s;  // ok
    let r2 = &s;  // ok
    let r3 = &mut s;  // not ok
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{}, {}, and {}", r1, r2, r3);

    // Note on scoping: Following is ok since r1 and r2 go out of scope
    // immediately after their last use
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used
    let r3 = &mut s;
    println!("{}", r3);

    // Can't create dangling pointer (see below)
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope, but is not dropped because it does not own the
    // reference (it is 'borrowed').
}

//fn change(s: &String) {
//    s.push_str(", world");
//}

fn change_mut(s: &mut String) {
    s.push_str(", world");
}

// error[E0106]: missing lifetime specifier
// this function's return type contains a borrowed value, but there is no value
// for it to be borrowed from.
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}  // s goes out of scope here, so we can't return a reference to it
