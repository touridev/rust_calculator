use std::io;

fn main() {
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");

    let num1: f64 = num1.trim().parse().expect("Please enter a number");
    let num2: f64 = num2.trim().parse().expect("Please enter a number");

    println!("You entered {} and {}", num1, num2);
}
