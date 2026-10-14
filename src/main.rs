use chrono::prelude::*;
use std::io;

fn main() {
    println!("Let's figure out when you will turn 100!");

    println!("\nWhat's your name?");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get your name!");

    let name = name.trim();
    let current_year: u32 = Utc::now().year().to_string().parse().expect("");
    let result = current_year + (100 - get_valid_age());

    println!("\n{name}, you will turn 100 in {result}!");
}

fn get_valid_age() -> u32 {
    loop {
        println!("\nHow old are you?");
        let mut age = String::new();

        io::stdin()
            .read_line(&mut age)
            .expect("Failed to get your age!");

        let age: u32 = match age.trim().parse() {
            Ok(num) if num <= 100 || 100 >= num => num,
            _ => {
                println!("You have entered invalid age!");
                continue;
            }
        };

        return age;
    }
}
