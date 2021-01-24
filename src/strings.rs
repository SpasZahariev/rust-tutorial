/*
There are TWO types of strings in Rust. 
Immutable (fixed length)
Growable (heap-allocated data structure)
*/

pub fn do_strings() {
    //Immutable
    let hello = "Hello";

    //Growable (I've made it mutable as well)
    let mut growable_hello = String::from("Hello");

    // Get length
    println!("My length is {}", growable_hello.len());

    // for single chars
    growable_hello.push('Z');

    // for whole words
    growable_hello.push_str("zzzzz");

    // Contains
    growable_hello.contains("ello");

    // Capacity in bytes
    println!("My capacity: {}", growable_hello.capacity());

    // Replace
    println!("{}", growable_hello.replace("z", "A"));

    // Loop through string by whitespace
    let my_sentance = "Hi Spas, How are you doin?";
    for word in my_sentance.split_whitespace() {
        println!("{}", word);
    }

    // Create string with specific capacity
    let mut my_string = String::with_capacity(10);
    my_string.push_str("abcd");

    // Assertion testing (if something fails the program immediately fails)
    assert_eq!(4, my_string.len());
    assert_eq!(10, my_string.capacity());
}