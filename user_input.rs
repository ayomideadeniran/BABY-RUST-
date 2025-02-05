// this project collect user input and print them out

use std::io;

fn main() {
    // This input is of first name 
    println!("Enter your First name:");
    let mut first_name = String::new();

    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to read line");

        // error handling in rust if first name is empty 
        if first_name.trim().is_empty() {
            eprintln!("First name cannot be empty.");
            return; 
        }
        
        // this  input is for last name 
        println!("Enter your Last name:");
        let mut  last_name = String::new();
        
        

        io::stdin()
        .read_line(&mut last_name)
        .expect("Failed to read line");
        
        // error handling in rust if first name is empty 
    if last_name.trim().is_empty() {
        eprintln!("Last name cannot be empty.");
        return; 
    }

    // this prints out your name
    println!("Hello, {}, {}!", first_name.trim(), last_name.trim()); 
}

