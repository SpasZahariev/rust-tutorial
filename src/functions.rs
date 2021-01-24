pub fn run() {
    greeting("Hello", "Spas");

    // Bind function values to variables
    let get_sum = add_numbers(5, 6);
    println!("{}", get_sum);

    // Closure (kind of like inline function)
    let n3: i32 = 10;
    let add_nums = | n1: i32, n2: i32| n1 + n2 + n3;
    println!("Sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add_numbers(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two
}