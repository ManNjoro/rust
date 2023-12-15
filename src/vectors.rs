// Fixed list of same data types
use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;
    numbers.push(5);
    numbers.push(6);
    numbers.pop();

    println!("{:?}", numbers);
    // single value
    println!("Single Value: {}", numbers[0]);
    println!("Vector Length: {}", numbers.len());

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}