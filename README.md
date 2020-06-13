# Rust Book

My examples and notes from learning Rust via [The Rust Programming Language](https://doc.rust-lang.org/stable/book) book.

## Notes

### Chapter 3.2 - Data Types

- Integer overflow checking is only included for debug builds, and will cause a panic at runtime. `--release` builds will not have this, and values will wrap instead.
