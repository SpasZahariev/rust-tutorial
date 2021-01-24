/*
Primitive Types --
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (unsigned/signed and number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a STATICALLY typed language. But there is also type inference so cool like python
pub fn run() {

    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 2342345235123;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater = 10 > 5;

    // I can also use unicode
    let face_emoji: char = '\u{1F600}';

    //Chars
    let character_one = 'a';

    println!("{:?}", (x, y, z, is_active, is_greater, character_one, face_emoji));
}