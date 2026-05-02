// simple calculator in Rust

fn main(){
    let num1 =10;
    let num2 =5;

    let sum =num1+num2;
    println!("The sum of {} and {} is {}", num1, num2, sum);

    let diff= num1- num2;
    println!("The difference of {} and {} is {}", num1, num2, diff);

    let product = num1 * num2;
    println!("The product of {} and {} is {}", num1, num2, product);

    let quotient = num1 / num2;
    println!("The quotient of {num1} and {num2} is {quotient}");
}