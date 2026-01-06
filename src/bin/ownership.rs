// // 1. Ownership Transfer
// // Write a function take_and_return that takes ownership of a String, appends "!" to it, and returns the ownership back. Try to use it in main so both before and after the function call, the data is usable.
// fn main() {
//     // println!("1. Ownership Transfer");

//     let mut s = String::from("hello");
//     s = take_and_return(s);
//     println!("{}", s);

// }

// fn take_and_return(mut s : String)-> String{
//     s.push_str("!");
//     s
// }

//------------------------------------------------------
// 2. Immutable Borrowing
// Create a function that accepts a reference to a vector of integers and returns the sum of its elements. Demonstrate passing a vector and using the vector after the function call.

// fn main() {
//     println!("2. Immutable Borrowing");

//     let data_vec = vec![1, 2, 3, 4, 5];
//     let sum = vec_sum(&data_vec);
//     println!("Sum of elements: {}", sum);
//     println!("Original vector after function call: {:?}", data_vec);
// }

// fn vec_sum(data: &Vec<i32>) -> i32 {
//     data.iter().sum()
// }

 
//------------------------------------------------------
// 3. Mutable Borrowing Rules
// Try to create two mutable references to the same variable in the same scope. Observe what error the Rust compiler generates and explain why.

// use std::string;


// fn main(){
//     println!("3. Mutable Borrowing Rules");
//     let  mut data = String::from("hello");
//     let r1 = &mut data;
//     // let r2 = &mut data;
//     // println!("{}, {}", r1, r2);
// }




//------------------------------------------------------
// 4. Slicing and Borrowing
// Write a function first_word that takes a string slice (&str) and returns the first word (slice) as another string slice. Show how this respects borrowing rules.

fn main (){
    println!("4. Slicing and Borrowing");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);
}

fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

//------

// Ownership: Key Concepts
// Each value in Rust has a single owner.

// When the owner variable goes out of scope, the value is dropped (memory freed).

// Ownership transfer is called a move; after moving, the original variable is no longer valid.

// Types that implement the Copy trait do not move but are copied.

// Example: Ownership Move

// let s1 = String::from("hello");
// let s2 = s1; // s1 moves to s2
// // println!("{}", s1); // ERROR: s1 is no longer valid
// println!("{}", s2);

// Borrowing: Key Concepts
// Borrowing means referencing data without taking ownership using & (immutable borrow) or &mut (mutable borrow).

// Many immutable borrows are allowed, but only one mutable borrow is allowed, and no immutable borrows can coexist with it.
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s;      // first immutable borrow
//     let r2 = &s;      // second immutable borrow
//     let r3 = &mut s;  // mutable borrow: ERROR

//     println!("{r1}, {r2}, and {r3}");
// }

// Borrowed references must always be valid (no dangling pointers).

// Example: Immutable Borrow
// rust
// fn print_length(s: &String) {
//     println!("Length: {}", s.len());
// }
// let my_string = String::from("hello");
// print_length(&my_string); // my_string still owns the data
