pub fn run() {
    // Print to console 
    println!("HELLO FROM THE PRINT.RS FILE");
    
    println!("Number={}", 1);
    println!("{} is from {}", "Spas", "Mama");

    // Positional Arguments
    println!("{0} is from {1} and {0} is the best at {2}", "Spas", "Earth", "code");

    // Named Arguments
    println!("{name} likes to play {game}", name = "Spas", game = "Computer Games");

    // Placeholder traits
    println!("Binary={:b} Hex={:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}