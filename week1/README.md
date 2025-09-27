# Week 1 - Name and Age Function

This is a simple Rust program that demonstrates basic function usage with string and integer parameters.

## Overview

The program contains a function `say_name_and_age` that takes a person's name and age as parameters and prints a friendly message to the console.

## Function Documentation

### `say_name_and_age(name: &str, age: u32)`

Prints a person's name and age to the console in a formatted message.

**Parameters:**
- `name` - A string slice (`&str`) that holds the person's name
- `age` - An unsigned 32-bit integer (`u32`) representing the person's age

**Output:**
The function prints a message in the format: "My name is [name] and I am [age] years old."

**Example Usage:**
```rust
say_name_and_age("Alice", 30);
// Output: My name is Alice and I am 30 years old.

say_name_and_age("Bob", 25);
// Output: My name is Bob and I am 25 years old.
```

## How to Run

1. Make sure you have Rust installed on your system. You can install it from [rustup.rs](https://rustup.rs/)

2. Navigate to the project directory:
   ```bash
   cd week1
   ```

3. Build and run the program:
   ```bash
   cargo run
   ```

   Or build first and then run:
   ```bash
   cargo build
   ./target/debug/week1
   ```

## Expected Output

When you run the program, you should see:
```
My name is Alice and I am 30 years old.
```

## Learning Objectives

This exercise demonstrates:
- Basic function definition in Rust
- Using string slices (`&str`) and integer types (`u32`)
- Rust's `println!` macro for formatted output
- Proper documentation comments using `///`
- Basic project structure with Cargo

## File Structure

```
week1/
├── Cargo.toml          # Project configuration
├── src/
│   └── main.rs         # Main source code
└── README.md           # This file
```