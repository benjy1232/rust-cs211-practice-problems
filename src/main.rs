// These are practice problems taken from my CS211 class just done using rust instead of C++
// I am going to try to follow rust conventions but this is so I can more or less just get a feel
// for what's happening.
use std::io;

// Make a function read user input as i32 because I don't want to write those lines down everytime
// they're being used
fn read_to_int() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line!");

    let input: i32 = input.trim().parse().expect("Input not integer");
    input
}

// Basically same thing as read_to_int except now using floats (double)
fn read_to_float() -> f32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line!");

    let input: f32 = input.trim().parse().expect("Input not integer");
    input
}

fn main() {
    // This range corresponds to the range of the first part of the practice problems set that can
    // be found in my CS211-Practice-Problems git repo
    println!("Please enter a number between 1 and 17");

    let input = read_to_int();

    match input {
        1 => hello_world(),
        2 => double_int(),
        3 => double_float(),
        4 => compare_int(),
        5 => do_math(),
        6 => find_remainder(),
        7 => compare_floats(),
        8 => sum_less_than_hundred(),
        _ => println!("Exiting program"),
    }
}

fn hello_world() {
    println!("Hello World!");
}

fn double_int() {
    println!("Please enter an integer:");
    let input = read_to_int();
    println!(
        "The number {} is double your input number {}",
        input * 2,
        input
    );
}

fn double_float() {
    println!("Please enter any number:");
    let input = read_to_float();
    println!(
        "The number {} is double your input number {}",
        input * 2f32,
        input
    );
}

fn compare_int() {
    println!("Please enter first number:");
    let x = read_to_int();
    println!("Please enter second number:");
    let y = read_to_int();
    println!("Result of comparison: {}", x == y);
}

fn do_math() {
    println!("Please enter first number (X):");
    let x = read_to_int();
    println!("Please enter second number (Y):");
    let y = read_to_int();

    println!("Sum of Numbers (X + Y): {}", x + y);
    println!("Difference of Numbers (X - Y): {}", x - y);
    println!("Difference of Numbers (Y - X): {}", y - x);
    println!("Product of Numbers (X * Y): {}", x * y);
    println!("Division of Numbers (X / Y): {}", x / y);
    println!("Division of Numbers (Y / X): {}", y / x);
}

fn find_remainder() {
    println!("Please enter first number:");
    let x = read_to_int();
    println!("Please enter second number:");
    let y = read_to_int();

    println!("Remainder (X % Y): {}", x % y);
}

fn compare_floats() {
    println!("Please enter first number:");
    let x = read_to_float();
    println!("Please enter second number:");
    let y = read_to_float();

    println!("Equality Check (X == Y): {}", x == y);
    println!("Non-Equality Check (X != Y): {}", x != y);
    println!("Less-Than Check (X < Y): {}", x < y);
    println!("Greater-Than Check (X > Y): {}", x > y);
    println!("Less-Than or Equal to Check (X <= Y): {}", x <= y);
    println!("Greater-Than or Equal to Check (X >= Y): {}", x >= y);
}

fn sum_less_than_hundred() {
    println!("Please enter first number:");
    let x = read_to_int();
    println!("Please enter second number:");
    let y = read_to_int();
    let out: String;

    if (x + y) < 100 {
        out = "true".to_string();
    } else {
        out = "false".to_string();
    }

    println!("Sum less than 100: {}", out);
}
