use std::io;
fn main() {
    println!("This is the beginning of this shit");

    // Get the first word
    println!("Enter the First word!");
    let first_word = get_word();

    // Get the second word
    println!("Enter the second word!");
    let second_word = get_word();

    // Get the third word
    // println!("Enter the third word!");
    // let third_word = get_word();
    // println!("The third word is: {}", third_word);

    // Compare the inputs
    if first_word == second_word {
        println!("🥂🥂🥂 The inputs are the same: {}", first_word);
    } else {
        println!(
            "🤧🤧🤧 The inputs are different. Input1: {}, Input2: {}",
            first_word, second_word
        );
    }
}



fn get_word() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read this line");
    input.trim().to_string()
}


