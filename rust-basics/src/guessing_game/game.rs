use rand::Rng;
use std::io;

pub fn run() {
    let secret = rand::thread_rng().gen_range(1..=10);

    println!("Guess a number between 1 and 10:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let guess: i32 = input.trim().parse().expect("Invalid number");

    if guess == secret {
        println!("🎉 Correct!");
    } else {
        println!("❌ Wrong! Number was {}", secret);
    }
}