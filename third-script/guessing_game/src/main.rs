
// Adds the input/output library.
// "std" is the Standard library of built-in items.
use std::io;

// Make the most used (prelude) parts of rand available in the program's scope.
use rand::prelude::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    // "let" creates the variable.
    // "mut" makes it mutable, so it can be changed.
    // String::new(); tells Rust to create a new instance of String.
    // :: defines that "new()" is a function that class String has.
    let mut guess = String::new();

    // Alternatively, we can not "use" and write the full std::io::stdin.
    // stdin is a type that can deal with standard inputs.
    // .read_line tells stdin to take the user input and store it in a string.
    // & means that we are referencing "guess".
    // So &mut guess is using a mutable reference to "guess".
    // .expect is another method of stdin. read_line should return a Result object (which is an
    // enum). Results can either be of variant "Ok" or variant "Err". If the return Result is of
    // variant "Err", then .expect is used.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Same as Python fstrings.
    println!("You guessed: {guess}");

    let x = 5;
    let y = 10;
    println!("x is {x}. y is {y}. y + 2 is {}", y + 2);
}
