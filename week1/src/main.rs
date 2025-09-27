// Write a single function that says your name and age.
fn main() {
    say_name_and_age("Alice", 30);
}

/// Prints a person's name and age to the console.
///
/// This function takes a name as a string slice and an age as an unsigned 32-bit integer,
/// then formats and prints them in a friendly message.
///
/// # Arguments
///
/// * `name` - A string slice that holds the person's name
/// * `age` - An unsigned 32-bit integer representing the person's age
///
/// # Examples
///
/// ```
/// say_name_and_age("Alice", 30);
/// // Output: My name is Alice and I am 30 years old.
///
/// say_name_and_age("Bob", 25);
/// // Output: My name is Bob and I am 25 years old.
/// ```
fn say_name_and_age(name: &str, age: u32) {
    println!("My name is {} and I am {} years old.", name, age);
}
