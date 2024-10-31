/// Using `use` to import the `io` module, bringing it into scope for use in this file.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // Declare a mutable variable of type String and initialize it as an empty string
    let mut guess = String::new();

    // Initialize the stdin method 
    io::stdin()
        //Read input and store it in the `guess` variable
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

