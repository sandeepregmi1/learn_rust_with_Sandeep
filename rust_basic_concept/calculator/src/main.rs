use std::io;

fn main() {
    let mut input = String::new();

    // First number
    println!("Enter first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num1: i32 = input.trim().parse().expect("Please enter a valid number");
    input.clear();

    // Second number
    println!("Enter second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num2: i32 = input.trim().parse().expect("Please enter a valid number");

    // Calculations
    let sum = num1 + num2;
    let diff = num1 - num2;
    let product = num1 * num2;

    let quotient = if num2 != 0 {
        Some(num1 / num2)
    } else {
        None
    };

    println!("\nResults:");
    println!("Sum: {}", sum);
    println!("Difference: {}", diff);
    println!("Product: {}", product);

    match quotient {
        Some(q) => println!("Quotient: {}", q),
        None => println!("Cannot divide by zero"),
    }
}