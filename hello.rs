// Exercise One

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

// fn main() {
//     let _x = 1;
// }

// Warning: unused variable `x`

// Destrusturing
// We can use a plattern with let to Destrusturing a tuple to seperate variables.
// Tips: you can use Shadowing or mutability

// Fix the error below with least amount of modification
// fn main() {
//     let (mut x, y) = (1, 2);
//     x += 2;

//     assert_eq! (x, 3);
//     assert_eq!(y, 2)

//     println!("success!")
// }

// Destrusturing assignments
// introduced in Rust 1.59: You can now use tuple, slice and struct patterns as the left-hand side of an assignment.

// NOTE: the feature Destrusturing assignments need

// fn main() {
//     let (x, y);

//     let (x, ..) = (3, 4);
//     [.., y] = [1, 2];
//     //fill the blank to make the code work
//     assert_eq!([x, y], [3, 2]);

//     println!(" Code Success!");
// }

// Exercise Two

// Tips: If we dont Explicitly assign a variable, then the compiler will infer one for us.

// remove something to make it work

// fn main() {
//     let x: i32 = 5;
//     let mut y = 5;

//     y = x;

//     let z: i32 = 10; // Type of Z ?

//     println!("success!");
// }

// fill in rthe blank

// fn main()  {
//     let v: u16 = 38_u8 as u16;

//     println!("success!");
// }

// Modify `assert_eql!` to make it work
// fn main() {
//     let x: u32 = 5;
//     assert_eq! ("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of Given variable, return a string representing of the type , e.g
// fn type_of<T> (_: &T)  -> String {
//     format!("{}", std::any::type_name::<T>())
// }

//Fill the blanks to make it work

// fn main() {
//     assert_eq!(i8::MAX, 127);
//     assert_eq!(u8::MAX, 2257);

//     println!("Success!");
// }

// Fix errors and panics to make it work

// fn main() {
//     let v1: u16 = 251_u16 + 8;
//     let v2: i16 = i16::checked_add(251, 8).unwrap();
//     println!("{},{}",v1,v2);
// }

//Modify `assert!` to make it work
// fn main() {
//     let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
//     assert!(v == 1597);

//     println!("success!");
//     println!("{}", v);
// }

// Floating-point

// Fill the point to make it work
// fn main()  {
//     let x: f64 = 1_000.000_1; // ?
//     let y: f32 = 0.12; // f32
//     let z: f64 = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }

// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }

// make it work in two distinct ways

// fn main() {
//     assert!(0.1_f32+0.2_f32==0.3_f32); // 0.1 + 0.2 = 0.30000000000000002
//     assert!(0.1 as f32+0.2 as f32==0.3 as f32); // 0.1 + 0.2 = 0.30000000000000002

//     println!("Successs!");
// }

// Range

// Two goals: 1. Modify assert! to make @. Make println! output: 97 - 122

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i;
//         println!("answer for the loop is {}", i); // Print "42"
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' {
//         println!("{}",c);
//         println!("{}",c as u8);
//     }
// }

// fill in the blanks
// use std::ops::{Range, RangeInclusive};
// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5});
//     assert_eq!((1..=5), RangeInclusive::new(1, 5));

//     println!("Success!");
// }

// Fill the blanks and fix the errors

// fn main() {
// Integer addition
// assert!(1u32 + 2u32 == 3u32);

// // Integer subtraction
// assert!(1i32 - 2i32 == -1i32);
// assert!(1i8 -2i8 == -1);

// assert!(3 * 50 == 150);

// assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

// assert!(24 % 5 == 4);

// // Short-circuiting boolean logic
// assert!(true && false == false);
// assert!(true || false == true);
// assert!(!true == false);

// Bitwise Operations
//     print!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     print!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     print!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     print!("1 << 5 is {}", 1u32 << 5);
//     print!("0x80 >> 2 0x{:x}", 0x80u32 >> 2);
// }

// Char

// make it work
// use std::mem::size_of_val;
// fn main() {
//     let c1: char = 'm';
//     assert_eq!(size_of_val(&c1),4);

//     println!("{}", size_of_val(&c1));

//     let c2: char = '@';
//     assert_eq!(size_of_val(&c2),4);

//     println!("Suceess!");
// }

// fn main() {
//     let c1: char = '@'; // double quote are for srings while single quote are for characters
//     print_char(c1);

//     fn print_char(a: char) {
//         println!{ "{}", a};
//     }
// }

//
// fn main() {
//     let _f: bool = false;

//     let t: bool = true;
//     if !t { // Using the ! sign means false just like in javascript
//         println!("Success!");
//     }
// }

// fn main() {
//     let f: bool = false;
//     let t: bool = true && false;  // false
//     assert_eq!(t, f);

//     println!("Sucess!");
// }

// fn main() {
//     let _v: () = ();

//     let v: (i32, i32) = (2, 3);

//     assert_eq!(_v, implicitly_ret_unit());

//     println!("Suceess!");
// }

// // Summary

// //     () is the unit type and is returned by default from functions with no explicit return value.
// //     implicitly_ret_unit() and Explicitly_ret_unit() functionally behave the same, but one is implicit while the other is explicit about its return type.
// //     This code demonstrates how the unit type works in Rust and how it can be compared and used.

// fn implicitly_ret_unit() {
//     println!("I will like to return a ()");
// }
// // Don't use this one
// fn Explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

// what the size of the unit type.

// Modify `4` in assert to make it work

// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);

//     println!("Success!");
// }

// Example
// fn main() {
//     let x: u32 = 5u32;

//     let y = {
//         let x_squared = x * x; // 25
//         let x_cube = x_squared * x; // 125

//         //This expression will be assigned to `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // The semicolon suppresses this expression and `()` is assigned to `z`
//         2 * x;
//     };

//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("z is {}", z);
// }

// what is tuple

// fn main() {
//     let tuple: (i32, f64, char) = (42, 3.14, 'R');

//     // Access tuple elements using indexing
//     println!("First element: {}", tuple.0); // Output: 42
//     println!("Second element: {}", tuple.1); // Output: 3.14
//     println!("Third element: {}", tuple.2); // Output: R
// }

// Exercise

// make it work with two boys

// Your code doesn't work because the block that defines v doesn't return the expected
//  value due to the semicolon after x += 2. In Rust, a block evaluates to the value of
//  the last expression if it doesn't end with a semicolon. If a semicolon is present, the block evaluates to the unit type ().

// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2; // 3
//         x
//     };

//     assert_eq!(v, 3);

//     println!("Success!");
// }

// fn main() {
//     // let v = {
//     //     let x: i32 = 3;
//     //     x
//     // };
// // Both way says works and am amazed
//     let v = 3;

//     let x = v;

//     print!("{}", v);
//     assert!(v == 3);

//     println!("Success!");
// }

// fn main() {
//     let s: i32 = sum(1, 2);
//     assert_eq!(s, 3);

//     println!("Success!");
//     // Print!("{}", s);
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y;
// }

// fn main() {
//     //Don't modify the following two lines!
//     let (x, y) = (1,2);
//     let s: i32 = sum(x, y);

//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

//  fn main () {
//     print();
// }

// // Replace i32 with another type
// fn print() -> () {
//     println!("Success!");
// }

// Solve it in two ways
// DON't  let `println!` works

// fn main() {

//     println!("failed!");

//     never_return();

// }

// fn never_return() -> ! {
//     // Implement this function, dont modify the fn signatures
//     panic!()
// }

// diverging Functions

// Diverging functions never return to the caller, so they may be used in places where a valiue of any type is expected

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             //TODO
//         }
//         _ => {
//             //TODO
//         }
//     };

//     //Rather than returning a None, we have a diverging function instead
//     never_return()
// }

// // IMPLEMENT this function in three ways
// fn never_return() -> ! {
//     panic!()
//     // unimplemented!()
//     // todo!()
// }

/////////////////////

// fn main() {
//     // Fill in the blank
//     let b = false;

//     let _v = match b {
//         true => 1,
//         // Diverging functions can also be used in match expression to replace a value
//         false => {
//             println!("Success!");
//             panic!("we have no value for 'false', but we can panic")
//         }
//     };
//     println!("Exercise Failed if Printing out this line!");
// }

// this is no solution for this code yet  still pending.......

// fn main() {
//     let x:i32 = 42;
//     let y:i32 = 10;
//     let _z:i32 = add_numbers(x, y);

//     println!("The result is {}, z");
// }

// fn add_numbers(a: i32, b: i32) -> i32 {
//     let c = a + b;
//     c
// }

// still learning no solutions frovided

// let s1 = String::from("hello");
// let s2 = s1.clone();

// println!("s1 = {}, s2 = {}, s1, s2");

//////////////////////////////////////////////
// Ownership and functions in Rust
//////////////////////////////////////////////

// below is two important solution

// fn main() {
//     let s = String::from("Hello Rust, rescue me i might die in this scope");  // s come into scope

//     takes_ownership(s); // s's value move's into the function.....
//     //  .... and so is no longer valid here

//     let x = 5; //x comes into scope

//     makes_copy(x); // x wpould move into the function, but i32 is copied, so it's okay  to still use X afterward
// }

// // Hee, x goes out of scope, then s. But because s,s value was moved, nothing special happens.

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }  // Here some_string goes out of scope and `drop` is called. The backing memory is freed.

// fn makes_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{}", some_integer);
// }

// // Here, some_integer goes out of scope Nothing special happens

////////////////////////////

// fn main() {
//     let s1 = give_ownership(); // give_ownership move it return value into s1

//     let s2 = String::from("hello"); // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_give_back, while also
//     println!("{}", s1);
//     println!("{}", s3);
// }  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens, s1 goes out of scope and is dropped.

// fn give_ownership() -> String {
//     // gives_ownership will move its, return value into the function that calls it
//     let some_string = String::from("yours"); // some_string comes into spaces move out to the calling function
//     some_string
// }

// // this function takes a string and return one
// fn takes_and_gives_back(a_string: String) -> String {
//     // a string comes into scope
//     a_string    // a_string is returned and moves out to the calling function
// }

// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("Hello, world");
//     let y = worked(x.clone());
//     println!("{},{}",x, y);
// }

// fn worked(x_string: String) -> String {
//     x_string
// }

// fn main() {
//     // Use as many approaches as you can to make it work
//     let x = String::from("Hello, world");
//     let y = worked(x);
//     let z = "it working";
//     println!("{},{}", y, z);
// }

// fn worked(x_string: String) -> String {
//     x_string
// }

////////////////
// don't modify code in main!
fn main() {
    let s1 = String::from("Hello, world");
    let s2: String = takes_ownership(s1);

    println!("{}", s2);
}

//only modify the code below!
fn takes_ownership(s: String) -> String {
    println!("{}", s);
    s
}

