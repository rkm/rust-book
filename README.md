# Rust Book

My examples and notes from learning Rust via [The Rust Programming Language](https://doc.rust-lang.org/stable/book) book.

## Notes

### Chapter 3.2 - Data Types

- Integer overflow checking is only included for debug builds, and will cause a panic at runtime. `--release` builds will not have this, and values will wrap instead.
- The `char` type is 4 bytes and represents a "Unicode Scalar Value".
- Array bounds checking is only done for "obvious" constants at compile-time:

```rust
// This will fail at compile-time
let a = [1, 2];
let el = a[10];

// This will panic at runtime
let a = [1, 2];
let idx = 10;
let el = a[idx];
```

### Chapter 3.3 - Functions

- The last expression in a function will be implicitly used as the return value. This really does have to be an _expression_ though, not a _statement_, i.e. it must not have a semicolon:

```rust
fn five() -> i32 {
    5
}
```

### Chapter 3.5 - Control Flow

- No implicit conversion to `bool` in `if` statements - condition must strictly be a `bool`.
