use std::io;

mod hello_world;
mod calculator;
mod guessing_game;

fn main() {
    loop {
        println!("\n🦀 Rust Basics Menu");
        println!("1. Hello World");
        println!("2. Calculator");
        println!("3. Guessing Game");
        println!("4. Exit");

        println!("Enter your choice:");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");

        match choice.trim() {
            "1" => hello_world::basic::run(),
            "2" => calculator::basic_calc::run(),
            "3" => guessing_game::game::run(),
            "4" => {
                println!("Goodbye 👋");
                break;
            }
            _ => println!("Invalid choice ❌"),
        }
    }
}