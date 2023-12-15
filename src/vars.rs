// Variables are immutable

pub fn run() {
    let name = "John";
    let mut age = 21;  // make it mutable
    println!("My name is {} and I am {}", name, age);
    age = 25;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let ( my_name, my_age ) = ("John", 21);
    println!("{} is {}", my_name, my_age);
}