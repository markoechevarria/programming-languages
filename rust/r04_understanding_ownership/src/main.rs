use std::any::type_name_of_val;


// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world");

    println!("Value of s: {}", s);
    println!("Type of s: {}", type_name_of_val(&s));

    let new_s = "Hello";
    println!("Value of new_s: {}", new_s);
    println!("Type of new_s: {}", type_name_of_val(&new_s));

    // first rule of ownership
    
    let s1 = "Hello World";
    let s2 = s1;
    println!("The value of s1: {s1} and s2: {s2}");

    let s1 = String::from("Hello World");
    let s2 = s1;
    println!("The value of s2: {s2}"); //  and s2: {s2}");

    let mut s1 = String::from("Hello world");
    println!("The value of s1: {s1}");
    s1 = String::from("Bye world");
    println!("The value of s1: {s1}");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // second rule of ownership

    let s1 = String::from("hello");
    takes_ownership(s1);

    let s2 = 5; 
    makes_copy(s2);

    // println!("s1 is {s1} and s2 is {s2}");


    // return values and scope
    
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);


    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");


    // References and Borrowing 
    // At any given time, you can have either one mutable reference or any number of immutable references.  
    // References must always be valid.
    
    let s1 = String::from("hello");
    length(&s1);
    println!("The value of s1 is {s1}");

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("The value of s1 is {s1}");


    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    // println!("{r1}, {r2}");
    
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{r1}, {r2}, and {r3}");

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");
    let r3 = &mut s;
    println!("{r3}");
    
    let s = no_dangle();


    // Slice type

    let mut s1 = String::from("First New Word");
    let s1_len = first_word(&s1);
    println!("the length of '{s1}' is {s1_len}");
    s1.clear();

    let mut s1 = String::from("First New Word");
    let (first_s2_len, second_s2_len) = second_word(&s1);
    println!("the second word index of '{s1}' is {first_s2_len} and {second_s2_len}");


    // String slices
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[..2];
    let slice = &s[3..len];
    let slice = &s[..];
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn length(some_string: &String) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

fn no_dangle() -> String {
    let s = String::from("hello world");
    s
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i) in bytes.iter() {
        println!("{i}");
    }

    for (i) in s.chars() {
        println!("{i}");
    }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> (usize, usize) {
    let bytes = s.as_bytes(); 
    let mut counter = 0;
    let mut first_index: usize = bytes.len();
    let mut second_index: usize = bytes.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if counter == 0 {
                first_index = i;
                counter = counter + 1;
            } else if counter == 1 {
                second_index = i;
            }
        } 
    }

    (first_index, second_index)
}
