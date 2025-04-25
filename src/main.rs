use std::io;
fn main() {
    println!("Hello, world!");
    println!("Nice to meet you world!");
   
    let mut name = String::new();
    let mut age = String::new();
   
    println!("What's your name?");
    io::stdin().read_line(&mut name).expect("Failed to read line");
   
    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("Failed to read line");

    let name = name.trim();
    let age: u32 = age.trim().parse().expect("Please enter a number");

    println!("Nice to meet you, {}! You are {} years old.", name, age);
}