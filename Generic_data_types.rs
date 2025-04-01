// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![340, 50, 25, 100, 65, 200];

//     let result = largest_i32(&number_list);
//     println!("The largest number is  {result}");
// }






// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     println!("Initial largest: {}", largest); // Print the initial largest value

//     for item in list {
//         println!("Current item: {}", item); // Print the current item being compared
//         if item > largest {
//             largest = item;
//             println!("New largest found: {}", largest); // Print when a new largest is found
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![50, 25, 100, 65, 200];

//     let result = largest_i32(&number_list);
//     println!("The largest number is  {result}");

//     let array = [1, 2, 3, 4, 5];
//     let largest_in_array = largest_i32(&array);
//     println!("The largest number in the array is: {}", largest_in_array);
// }










// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {result}");

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest_char(&char_list);
//     println!("The largest char is {result}");
// }






// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn y(&self) -> &T {
//         &self.y
//     }
// }


// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }



// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
//     println!("p.x = {}", p.y());

// }

















// Define a trait named `Summary`
pub trait Summary {
    fn summarize(&self) -> String;
}

// Implement the trait for `NewsArticle`
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - {} ({})", self.headline, self.author, self.location)
    }
}

// Implement the trait for `Tweet`
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Function that accepts any type that implements `Summary`
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Main function to test our code
fn main() {
    let article = NewsArticle {
        headline: String::from("Rust is amazing!"),
        location: String::from("San Francisco, CA"),
        author: String::from("John Doe"),
        content: String::from("Rust is a systems programming language that prevents segfaults."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Rust makes memory safety easy!"),
        reply: false,
        retweet: false,
    };

    println!("News Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}
