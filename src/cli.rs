
// use std::env;

pub fn run() {
    //get arguments from command line
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();

    // println!("{:?}", args);

    if command == "hello" {
        println!("Hello how are you?!");
    } else {
        println!("Nope I don't care about {}", command);
    }
}