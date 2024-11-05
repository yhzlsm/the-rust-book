**Programming a Guessing Game:**

I'm learning Rust from scratch and the objective is to record my learning and contributions in open-sources projects.

I'm starting with The Rust Book (Abridged) to get an overview of how rust works. The goal is to understand the basics and starting make some exercises.

**Notes of how Rust works:**

- The Rust work with a concept called prelude. That means every Rust program has list of things that Rust imports. With this we can choose import implicit or explicitly.

**Examples in our Guessing Game**;

**1 - Explicit form:**

```rust
use std::io;

fn main() {
	println!("Guess the number");

	println!("Please input your guess.");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {guess}");
}
```

**2- Implicit form:**

```rust
fn main() {
	println!("Guess the number");

	println!("Please input your guess.");

	let mut guess = String::new();

	std::io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {guess}");
}
```

- Variables in Rust are immutable. For make them mutable to manipulate we use the `mut` after the `let`.

```rust
let mut guess = String::new();
```

- Many types in Rust are implemented with a `new` constructor. In this case, we use `String::new()` to create an empty string. Here, `new` is an associated function defined on the `String` type, allowing us to instantiate new `String` objects.

- The `&` symbol is a reference to the variable. It’s very important in Rust for several reasons:
  1.  It helps protect against accessing something that doesn’t exist or is not available.
  2.  It allows sharing data without transferring ownership.
  3.  It protects against unauthorized access or modification of data.

**Obs:** When we pass a value in rust without using a reference we transfer the ownership of the values to the called function.

What's happen when a function fail in Rust?
Return `Result` 1. `Ok` - Success case - Could contain some information. 2. `Err` - Signal an error has occurred - Contain the reason why the operation failed.

- Trait = Is a set of methods.
- Enum is Rust can carry information. Ex: `Result`.

The type inference is Rust work "like magic". Ex: If we compare `cmp` two strings and one of that has a type, Rust will infere the type for the other variable, making sure that we don't compare mismatch types.
