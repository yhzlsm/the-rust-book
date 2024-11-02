/// Using `use` to import the `io` module, bringing it into scope for use in this file.
use std::io;
/// Add trait Rng from the rand library to the scope.
use rand:Rng;

fn main() {
    println!("Guess the number!");

   /// Add rand crate to generate random number between 1 and 100 included. If I want to generate range between 1 and 99, I simply write like this .gen_range(1..100);
   
   // We call thread_rgn() from crate rand 
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);

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

