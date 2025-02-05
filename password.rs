use std::io;

fn main() {
    println!("This is the beginning of this shit");

    loop {
        
        // Get the first word
        println!("Enter the Password!");
        let first_word = get_word();
        
        // Get the second word
        println!("Enter the Comfim Password!");
        let second_word = get_word();
        
        // Compare the inputs
        if first_word == second_word {
            println!("The password match! Exiting the loop.");
            break; // Exit the loop when the inputs match
        } else {
            println!("The details do not match. Please try again.");
        };
    }
}
    
fn get_word() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read this line");
    input.trim().to_string()
}

