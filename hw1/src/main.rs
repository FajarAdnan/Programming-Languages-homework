// Beginning of the guessing_game (code is also in the guessing_game project folder)

// Allows us to accept user input
use std::io;

fn main() {
    println!("Guess the number!");
	println!("Please input your guess.");

	// let foo = bar;
	// let foo = 5; // immutable
	// let mut bar = 5; // Mutable/Changeable

	// Creates a mutable (changeable) variable
	let mut guess = String::new();

	// Error handling
	io::stdin().read_line(&mut guess)
		.expect("Failed to read line");

    // Print the user's input
    println!("You guessed: {}", guess);
}
