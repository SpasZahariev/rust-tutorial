// this is my dynamic array (ArrayList)

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    println!("{:?}", numbers);

    //Push
    numbers.push(21);
    numbers.push(22);

    println!("{:?}", numbers);

    //Pop last value
    numbers.pop();
    println!("{:?}", numbers);

    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Slicing arrays
    //notice this type means a reference to an array of ints
    let slice: &[i32] = &numbers;
    println!("Slice is {:?}", slice);

    //kind of like python
    let slice_of_two: &[i32] = &numbers[0..2];
    println!("Slice Of Two is {:?}", slice_of_two);

    // Looping through vector
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Looping and Mutate through vector
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number: {:?}", numbers);
}