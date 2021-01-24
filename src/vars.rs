// Variables hold primitve data OR references to data (Just like Java)
// Variables are IMMTABLE by default
// Rust is a block-scoped language. (if it is created inside a function you cant use it outside of the function)

pub fn goFunction() {
    let name = "Spas";
    let age = 23;

    println!("My name is {} and I am {}", name, age);

    // Define constants (not used alot since everything is immutable)
    const ID: i32 = 123;

    // Assign multiple vars
    let ( my_name, my_age) = ("Spas2", 24);
    println!("my name is {}, my age is {}", my_name, my_age);
}