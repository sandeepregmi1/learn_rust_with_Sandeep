fn main() {
    println!("Hello, world!");

    greet();
    intro();
}

// using variable
fn greet() {
    let name = "Sandeep";
    println!("Hello, {}!", name);
}

// Multiple values
fn intro() {
    let name = "Sandeep";
    let age = 10;

    println!("My name is {} and I am {}", name, age);

    named_format();
}

// Named formatting
fn named_format() {
    let name = "Sandeep";
    let age = 10;

    println!("My name is {name} and I am {age}");

      pretty_print();

    debug_print();
}

// Debug printing
fn debug_print() {
    let numbers = [1, 2, 3];

    println!("Array: {:?}", numbers);

  
}

// Pretty printing
fn pretty_print() {
    let numbers = [1, 2, 3];

    println!("Pretty: {:#?}", numbers);
}