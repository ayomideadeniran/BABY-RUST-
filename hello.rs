// Fix the error below with least amount of modification to the code

// fn main() {
//     let x: i32 = 6; // Unitialized but used, ERROR !
//     let _y: i32; // Unitialized but also unused, only a warning !

//     assert_eq!(x, 6);
//     println!("success!")
// }




// Use mut to mark a variable as mutable
// fill the blanks in the code to make it compile



// fn main() {
//     let mut x = 1;
//     x += 2;

//     assert_eq! (x, 3);
//     println!("Success!")
// }




// A Scope is the range within the program for which the item is valid

// fn main() {
//     let x: i32 = 10;

//     let mut y: i32 = 5;

//     y = y + 100;

//     {
//         let y: i32 = 5; 
//         println!("The value of x is {} and of y is {}", x, y);
//     }

//     println!("The value of x is {} and value of y is {}", x, y);
// }




//  Fix the error with the use of define_x 

// fn main() {
//     define_x()
// }

// fn define_x() {
//     let x = "hello";

//     println!("{}, world", x);
// }



// Shadowing 

//you can declare a new variable with the same name ad a previous,
//  here we can say **the first one is shadowed by the second one. 

//Only modify "assert_eq" to make the "print!" work (print `42` in Terminal)


// fn main() {
//     let x: i32 = 5;

//     {
//         let x = 12;
//         assert_eq!(x,12);
//     }

//     assert_eq!(x, 5);

//     let x = 42;
//     println!("{}", x); // Print "42"
// } 



// Remove a line in the code to make it compile 

// fn main() {
//     let mut x: i32 = 1; 
//     x = 7;
//     // Shadowing re-binding
//     let mut x = x;  // this line is not needed just fancy and it did not stop anything from working.
//     x += 3;

//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!";

//     println!("Ayomide!");
// }


// unused variable

// Fix the warning below with:

// Only one Solution
// Two distinct solutions 


// NOTE: none of the solutions is to remove the line let x = 1


// #[allow(unused_variables)]

fn main() {
    let _x = 1;
}

// Warning: unused variable `x`