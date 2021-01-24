// Arrays - fixed (EXACTLY THIS Much ELEMENTS) size and elements have the Same type!

//importing library so I don't have to type out std::mem all the time
use std::mem;

pub fn run() {
    let numbers: [i32; 6] = [1, 2, 3, 4, 5, 6];

    println!("{:?}", numbers);

    // Arrays are stack allocated (get a footprint of how much memory it takes)
    // notice that '&' passes a reference to the var
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slicing arrays
    //notice this type means a reference to an array of ints
    let slice: &[i32] = &numbers;
    println!("Slice is {:?}", slice);

    //kind of like python
    let slice_of_two: &[i32] = &numbers[0..2];
    println!("Slice Of Two is {:?}", slice_of_two);
}