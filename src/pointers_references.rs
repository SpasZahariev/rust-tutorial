/*
With non-primitive types, if you assign another variable to a piece of data, the first variable will no longer hold that value.
You will have to use a reference (&) if you want multiple things pointing to a resource
*/

pub fn run() {
    // Primitive array
    let array_one = [1,2,3];
    let array_two = array_one;

    println!("Values: {:?}", (array_one, array_two));



    // Vector (non-primitive)
    let vector_one = vec![4,5,6];
    let vector_two = &vector_one;

    println!("Values for vectors: {:?}", (&vector_one, vector_two));
}