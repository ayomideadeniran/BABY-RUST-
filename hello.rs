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
// fn main() {
//     let s1 = String::from("Hello, world");
//     let s2: String = takes_ownership(s1);

//     println!("{}", s2);
// }

// //only modify the code below!
// fn takes_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// //Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // Convert String to vec
//     // let _s = s.clone().into_bytes();    //Instead of using clone just comment out this line and the code will work just fine
//     let _s = s.as_bytes();    //Instead of using clone just comment out this line and the code will work just fine
//     println!("{}", s);
//     s
// }

// i Need solution to thsi Equation

// let s = String::from("hello");
// let bytes = s.into_bytes();

// assert_eq!(&[104, 101, 108,108, 111][..], &bytes[..]);

// Fix the error without removing code line

// fn main() {
//     let s = String::from("hello, world");

//     print_str(s);

//     println!("{}", s);
// }

// // fn print_str(s: String) -> String {
// //     println!("{}", s);
// //     s
// // }

// fn print_str(s: String) {
//     println!("{}", s);
// }

// // Don't use clone, use copy instead
// fn main() {
//     // let x = (1, 2, (), "hello".to_string());
//     // let y = x.clone();
//     let x: (i32, i32, (), &str) = (1, 2, (), "hello");
//     let y: (i32, i32, (), &str) = x;
//     println!("{:?}, {:?}", x, y);
// }

// fn main() {
//     let x = (1, 2, (), "hello"); // All types in this tuple implement `Copy`
//     let y = x; // Copy semantics are used here

//     println!("{:?}, {:?}", x, y);
// }

// Mutability
// mutabulity can be changed when ownership is transferred

// fn main() {
//     let s = String::from("hello, ");
//     // let s = "hello";

//     // Modify this line only !
//     let mut s1 = s;

//     s1.push_str("world");

//     println!("{}", s1);
//     // so what we did above was that we push ("World") to s

//     println!("Success!");
// }

// fn main() {
//     let x = Box::new(5);

//     let mut y: Box<i32> = Box::new(1);   // Implement this line, don't change other lines!

//     *y = 4;

//     assert_eq!(*x, 5);

//     println!("Success!");
// }

// Partial move
// within the destructuring of a singlr vaariable, both by-move and by-reference pattern bindings can be used at the same time.
//  Doing this will result in a partial move of the variable, which means that part of the variable will be moved while other parts stay.
//  In Such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person: Person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // `name` is moved out of person, but `age` is referenced
//     let Person { ref name, ref age } = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     //Error! borrow of partially moved value: `person` partial move occurs
//     println!("The person struct is {:?}", person);

//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age );
// }

// fn main() {
//     let t: (String, String) = (String::from("hello"), String::from("world"));

//     let _s = t.0;

//     //Modify this line only, don't use `_s`
//     println!("{:?}", t.1);
// }

// fn main() {
//     let t: (String, String) = (String::from("hello"), String::from("world"));

//     //Fill the blanks
//     let (s1, s2) = t.clone();

//     println!("{:?}, {:?}, {:?}", s1, s2, t); //
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{}", s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(" world");

// }

// let mut s = String::from("Hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

// println!("{},{}, and {}", r1, )

// let mut s = String::from("hello");

// {
//     let r1 = &mut s;
// }   // r1 goes out of scope here, so we can make a new reference with no problems.

// let r2 = &mut s;

// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// println!("?{} and {}", r1, r2);
// // variables r1 and r2 will not be used after this point

// let r3 = &mut s; // no problem
// println!("{}, r3");

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// this line of codes give me more understaning of how to print the main value of integers

// fn main() {
//     let x: i32 = 5;
//     // Fill the blank
//     let p: &i32 = &x;

//     let o: &i32 = &p;

//     println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac
//     println!("the memory address of x is {:o}", o); // One possible output: 0x16fa3ac
// }

// fn main() {
//     let x = 5;
//     let y = &x;

//     // Modify this line only
//     assert_eq!(5, *y);

//     println!("Success!");
// }

// Fix error
// fn main() {
//     let s = String::from("hello, ");

//     borrow_object(&s);

//     println!("{}", s);
// }

// fn borrow_object(s: &String) {}

// Fix error
// fn main()  {
//     let mut s: String = String::from("hello, ");

//     push_str(&mut s);

//     println!("Success!");
// }

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// fn main() {
//     let mut s = String::from("hello, ");

//     // Fill the blank to make it work
//     let p = &mut s;

//     p.push_str("world");

//     println!("Success!");
// }

// fn main()  {
//     let c = '@';

//     let r1 = &c;
//     // Fill the blank, dont change other code
//     let ref r2 = c;

//     assert_eq!(*r1,  *r2);

//     // Check the equality of the two address strings
//     assert_eq!(get_addr(r1), get_addr(r2));

//     println!("Success!");
// }

// // Get memory address string
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// Borrowing rules

// Removes something to make it work
// Don't remove a whole line !

// fn main() {
//     let s = String::from("hello");

//     let r1 = &s;
//     let ref r2 =  s;

//     println!("{}, {},", r1, r2);

//     println!("s: {}", s); // This line really show that s still have the value and it 100% Borrowed

//     println!("Success!");
// }

// Mutability
// Error: Borrow an immutable object as mutable

// fn main() {
//     // Fix error by modifying this line
//     let mut s = String::from("hello, ");

//     borrow_object(&mut s);

//     println!("Success!");
// }

// fn borrow_object(s: &mut String) {
//     println!("{}", s);
// }

// Borrow a mutable object as immutable

// This code has no errors!

// My review = am so surprice it works WOW this is massive......

// fn main() {
//     let mut s = String::from("hello, ");

//     borrow_object(&s);

//     s.push_str("world");

//     println!("Success!");
//     println!("{}", s);
// }

// fn borrow_object(s: &String) {
//     println!("{}", s)
// }

//comment one line to make it work

// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     r1.push_str("world");
//     // let r2 = &mut s;
//     // r2.push_str("!");

//     println!("{}", r1);
// }

// fn main() {
//     let mut s = String::from("hello, ");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     // Add one line below to make a compiler error: cannot borrow `s` as mutable  more than once at a time
//     // You can't use r1 and r2 at the same time
//     println!("{}, {}", r1, r2)
// }

// my ideal of rust compiler is that
// it is the best compiler i ever work with //C + phython + Javascript + ReactJS//

// STRING vs. &STR

// . A String is a heap-allocated string type that own its contents and is mutable

// .A &str is an immutable sequence of UTF-8 bytes in memory, it does not own the underlying data and is immutable

// .Think of Str as a view on a sequence of characters (stored as UTF-8bytes) in memory

// Use &str  if you want to a view of a string
// &str is more lightweight and efficient than String
// Use String if you need to own the data and be able to mutate it

// Example String slice
// The string slice world points to a sequence of characters stored on the heap

// let s:String = String::from("hello world");

// let hello = &s[0..5];
// let world = &[6..11];

// String

// the type of string literal "hello, world" is &str, e.g let s: &str = "hello, world".

// Str and &str

// we can't use str type in normal ways, but we can use &str.

// Fix error without adding new line
// fn main() {
//     let s: &str = "hello";
//     // let s:String = String::from("hello world");

//     println!("Success");
// }

// We can only use str by boxing it, & can be used to convert Box<str> to &str

// Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "Hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}", s)
// }

// String
// String type is defined in std and stored as a vector of bytes(vec), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null

// fill in the blank
// fn main() {
//     let mut s: String = String::from("");
//     s.push_str("hello, world");
//     s.push('!');

//     assert_eq!(s, "hello, world!");

//     println!("{}", s);

//     println!("Success!");
// }

// Fix all errors without adding newline
// fn main() {
//     let mut s:String = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";  // am surprice that i can add to a string using this line that is sick isnt it?

//     println!("{}", s);
// }

// Repalce can be used to replace substring

// Fill the blank

// fn main() {
//     let s: String = String::from("I like dogs");
//     // Allocate new memory and store the modified string there

//     let s1 = s.replace("dogs", "cats"); //  i can see that i have to print similar message.

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

// More String method can be found under String module.

// You can concat a string with &str, and String's ownership can be moved to another variable

// Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello, ");
//     let s2 = String::from("world!");
//     // let s3 = s1.clone() + &s2;
//     let s3 = s1 + s2.as_str(); // &String -> $str
//     assert_eq!(s3, "hello, world!");
//     println!("{}", s3);
//     // println!("{}", s1);
// }

// &str and String
// Opposite to the seldom using of str, &str and String are used everywhere!

// &str can be converted to string in two ways

// Fix error with at least two solutions

// fn main() {
//     let s: &str = "Hello, world";
//     // greetings(s.to_string())  // &str -> String
//     // greetings(String::from(s))  // &str -> String
//     greetings(s.to_owned())  // &str -> String
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }





// We can use String::from or to_String to convert a &str to string

// We can use approches to fix the error and without adding a new line



// fn main() {
//     let s: String = "hello, world".to_string();
//     let s1: &str = &s;  // &String -> &str
//     let s1: &str = s.as_str(); // &String -> &str

//     println!("Success!");
// }




// String escapes

// fn main() {
//     // you can use escapes to write bytes by their hexadecimal values
//     // Fill the blank below to show "i'm writing Rust"
//     let byte_escape = "i'm writing Ru\x73\x74!";
//     println!("What are you doing \x3f (\\x3F means ?) {}", byte_escape);

//     //  ...Or Unicode code points.
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!("Unicode character {} (u+211D) is called {}",
//   w                                  unicode_codepoint, character_name);

//     let long_string = "String literals
//                          can span multiple lines.
//                         The linebreak and indentation here \
//                         can ne escaped too!";
//     println!("{}", long_string);
// }



// Sometimes there are just too many characters that need to be escaped or
// much more convenient to write a string out as-is. This is where raw string literals come into play.

// fill in the blank and fix the errors

// this code is not working cause no solution to follow

// fn main() {
//     let raw_str = r"Escapes don't work here: \x3f \u\{211D}";
//     // modify above line to make it work 
//     assert_eq!(raw_str, "Escape don't work here: ? R");

//     // If you need quote in a raw string, add a pair of #s
//     let quotes = r#"And then i said: "There is no escape!""#;
//     println!("{}", quotes);

//     // If you need quote "# in your string, just use more #s in the delimiter.
//     // You can use up to 65535 #s.
//     let delimiter = r###"A string with" # in it. And even "##!"###;
//     println!("{}", delimiter); 
// }



// String index

// You can't use index to Access a char in a string, but you can use slice &s1[start..end].




// fn main() {
//     let s1:String = String::from("hi,#@");
//     let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte
//     assert_eq!(h, "h");

//     let h1 = &s1[3..4];  // Modify this line to fix the error, tips: `@` take 3

//     assert_eq!(h1, "#");

//     println!("{}", h1);

// }


// fn main() {
//     // Fill the blank to print each char in "#$, %^"
//     for c in "#$, %^".chars() {
//         println!("{}", c)
//     }
// }


// fn main() {
//     // Fill in the blank with proper array type
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];

//     // Modify the code below to make it work
//     assert!(arr.len() == 5);

//     println!("Success!");
// }



// fn main() {
//     //We can ignore parts of the array type or even the whole type, let the compiler infer it for us
//     let arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c'];

//     //Fill in the blank 
//     //Array are stack allocated, `std::mem::size_of_val' return the bytes which
//     // A char take 4 bytes in rust: Unicode Char
//     assert!(std::mem::size_of_val(&arr) == 12);

//     println!("Success!");
// }


// fn main() {
//     // Fill the blank 
//     let list: [i32; 100] = [1; 100];  // This is contain [1 to 100]

//     assert!(list[0] == 1);
//     assert!(list.len() == 100);

//     println!("Success!");
//     println!("{:#?}", list)
// }

// All elements in an array mush be of the same type 


// fn main() {
//     // Fix the error 
//     let _arr: [i32; 3] = [1, 2, 3];

//     println!("Success!")
// }


// Indexing start at 0. 

fn main() {
    let arr: [char; 3] = ['a', 'b', 'c'];

    let ele = arr[1]; // Only modify this line to make the code work!

    assert!(ele == 'b');

    println!("Success!");
}

// Out of bound indexing cause panic

// Fix the error 
fn main() {
    let names [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` return an Option <T>, it's safe to use
    let name0 = names.get(0).unwrap()

    //But indexing is not safe
    let _namel = &names[2];

    println!("Success!");
}


