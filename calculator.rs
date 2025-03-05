use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    // Get the first number
    println!("Enter the first number:");
    let num1 = get_input_as_f64();

    // Get the operator
    println!("Enter an operator (+, -, *, /):");
    let operator = get_input_as_string();

    // Get the second number
    println!("Enter the second number:");
    let num2 = get_input_as_f64();

    // Perform the calculation
    let result = match operator.as_str() {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => {
            println!("Error: Invalid operator.");
            None
        }
    };

    // Display the result
    match result {
        Some(value) => println!("The result is: {}", value),
        None => println!("Could not calculate the result."),
    }
}

fn get_input_as_f64() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

fn get_input_as_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

