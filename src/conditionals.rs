pub fn run() {
    let age = 18;
    let checked_id: bool = false;

    // if or else

    if age >= 21 && checked_id {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 || checked_id {
        println!("Bartender: One of these match up so you good!");
    } else {
        println!("Bartender: You shall not pass!");
    }

    // shorthand IF
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of Age: {}", is_of_age);
}