fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);

    // Tuples and pattern matching / destructuring
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);

    // Arrays
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];  // 5-element array of 3s
}
