# The Rust Programming Language Book: Chapter 2

From the Chapter 2 exercise found here: [https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)

Covered:
- Using Cargo
  - `cargo new`
  - `cargo check`
  - `cargo run`
  - `cargo build`
  - `cargo build --release`
  - Adding a new dependency to the Cargo.toml file

- Rust basics
  - Using `let` to define variables and `mut` to indicate mutable variables
  - Defining a variable's type with its `new()` method e.g. `String::new()`
  - Using `&` to denote a reference to a variable, allowing it to be accessed from multiple parts of the code
  - Using `std::io` for input/output functionality e.g. `io::stdin().read_line(&mut guess)`
  - Using `.expect()` to crash the application on error
  - Using the `loop` keyword to create an infinite loop which is only terminated using the `break` keyword
  - Using pattern matching to handle different cases e.g.

```
match guess.cmp(&secret_number) {
  Ordering::Less => println!("Too small!"),
  Ordering::Greater => println!("Too big!"),
  Ordering::Equal => println!("You win!"),
}
```
  - Casting a string type value to an unsigned 32-bit integer with error handling e.g.

```
let guess: u32 = match guess.trim().parse() {
  Ok(num) => num,
  Err(_) => continue,
};
```