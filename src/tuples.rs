// max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("John", "Ronga", 21);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}