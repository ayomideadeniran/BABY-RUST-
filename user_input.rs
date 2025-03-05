// // this project collect user input and print them out

// use std::io;

// fn main() {
//     // This input is of first name 
//     println!("Enter your First name:");
//     let mut first_name = String::new();

//     io::stdin()
//         .read_line(&mut first_name)
//         .expect("Failed to read line");

//         // error handling in rust if first name is empty 
//         if first_name.trim().is_empty() {
//             println!("First name cannot be empty.");
//             return; 
//         }
        
//         // this  input is for last name 
//         println!("Enter your Last name:");
//         let mut  last_name = String::new();
        
        

//         io::stdin()
//         .read_line(&mut last_name)
//         .expect("Failed to read line");
        
//         // error handling in rust if first name is empty 
//     if last_name.trim().is_empty() {
//         println!("Last name cannot be empty.");
//         return; 
//     }

//     // this prints out your name
//     println!("Hello, {}, {}!", first_name.trim(), last_name.trim()); 
// }



            // Updated code 




// So i think of Writing a smarter version of this app in the next edit
// making it more easy to read and understand, if you will like to manipute the input
// short and reusable code
use std::io;

fn main() {
    println!("Input your first name:");
    let first_name = get_word();
    println!("Input your last name:");
    let last_name = get_word();
    println!("Hello, {} {}!", first_name, last_name)
}



fn get_word() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read this line");
    input.trim().to_string()
}
