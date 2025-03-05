use std::io;


struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}


fn main() {
    println!("\n=== User Registration Form ===\n");

    println!("Enter First Name:");
    let first_name = get_word();

    println!("Enter Last Name:");
    let last_name = get_word();

    println!("Enter Email:");
    let email = get_word();
    
    // Password loop
    let password = loop {
        println!("\nEnter Password:");
        let first_word = get_word();
        
        println!("Confirm Password:");
        let second_word = get_word();
        
        if first_word.len() < 6 {
            println!("Password must be at least 6 characters long. Please try again.");
            continue;
        }
        
        if first_word == second_word {
            println!("\n✓ Passwords match!");
            break first_word;
        } else {
            println!("✗ Passwords do not match. Please try again.");
        }
    };

    // Create user
    let user = User {
        first_name,
        last_name,
        email,
        password,
    };

    // Registration success message
    println!("\n=== Registration Successful! ===");
    println!("Welcome, {} {}!", user.first_name, user.last_name);
    println!("Verification email sent to: {}", user.email);
}


fn get_word() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}



