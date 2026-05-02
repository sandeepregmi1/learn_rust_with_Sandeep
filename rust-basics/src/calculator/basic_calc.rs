use std::io;

pub fn run() {
    println!("🧮 Simple Calculator");

    // Input first number
    let mut num1 = String::new();
    println!("Enter first number:");
    io::stdin().read_line(&mut num1).expect("Failed");

    let num1: f64 = match num1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number ❌");
            return;
        }
    };

    // Input operator
    let mut op = String::new();
    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut op).expect("Failed");

    let op = op.trim();

    // Input second number
    let mut num2 = String::new();
    println!("Enter second number:");
    io::stdin().read_line(&mut num2).expect("Failed");

    let num2: f64 = match num2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number ❌");
            return;
        }
    };

    // Perform calculation
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Cannot divide by zero ❌");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Invalid operator ❌");
            return;
        }
    };

    println!("✅ Result: {}", result);
}