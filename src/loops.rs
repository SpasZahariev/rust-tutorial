
pub fn run() {
    let mut count = 0;

    // Infinite Loop
    // loop {
    //     count +=1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While loop (FizzBuzz)
    let mut count_two: u8 = 0;
    
    while count_two <= 120 {
        if (count_two % 10 == 0) {
            println!("{}", count_two);
        }
        count_two += 1;
    }

    // For Range like python

    for x in 0..100 {
        println!("{}", x);
    }
}