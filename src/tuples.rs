/*
Group values together (different types allowed too)
Max 12 elements
 */

pub fn run() {
    // I am using string literals here
    let person: (&str, &str, i8) = ("Spas", "Zahariev", 23);

    println!("{} has the surname {} and is {}", person.0, person.1, person.2);
}