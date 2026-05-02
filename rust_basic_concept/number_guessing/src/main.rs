use std::io;
use rand::Rng;   // Importing the Rng trait for random number generation from the rand crate in Cargo.toml

fn main() {
    println!("🎯 Welcome to the Number Guessing Game!");
    println!("Guess a number between 1 and 100.\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);  // Generate a random number between 1 and 100

    loop {   // Infinite loop until the user guesses correctly


        let mut guess = String::new();

        println!("Enter your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        if guess < secret_number {
            println!("📉 Too low!");
        } else if guess > secret_number {
            println!("📈 Too high!");
        } else {
            println!("🎉 Correct! You guessed it!");
            break;
        }
    }
}