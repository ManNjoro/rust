pub fn run() {
    let hello = "Hello"; // primitive str
    let mut hello2 = String::from("Hello "); // String - growable
    // Get length
    println!("Length: {}", hello.len());
    hello2.push('W'); // 1 char only
    hello2.push_str("orld!");

    // capacity in bytes
    println!("Capacity: {}", hello2.capacity());

    // is Empty
    println!("Is Empty: {}", hello2.is_empty());

    // Contains
    println!("Contains 'World' {}", hello2.contains("World"));

    // Replace
    println!("Replace: {}", hello2.replace("World", "There"));

    // Loop thru string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
}