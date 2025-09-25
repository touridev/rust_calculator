use std::io;

fn main() {
    loop {
        println!("Enter the first number (or type 'quit' to exit):");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1 = num1.trim();
        if num1.eq_ignore_ascii_case("quit") {
            break;
        }
        let num1: f64 = match num1.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("Enter the second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("Choose an operation (+, -, *, /) or type 'quit' to exit:");
        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read line");
        let op = op.trim();
        if op.eq_ignore_ascii_case("quit") {
            break;
        }

        let result = match op {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 { num1 / num2 } else { 
                    println!("Cannot divide by zero!"); 
                    continue; 
                }
            }
            _ => {
                println!("Invalid operation!");
                continue;
            }
        };

        println!("Result: {}\n", result);
    }

    println!("Goodbye!");
}
