use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let console_in = io::stdin();
    let mut rng = rand::thread_rng();

    println!("Welcome to the guessing game.");

    let random_number: u32 = rng.gen_range(1..=100);
    // println!("Your random number is {}.", random_number); // DEBUG

    loop {
        println!("Please enter your guess.");
        let mut user_input = String::new();
        console_in
            .read_line(&mut user_input)
            .expect("Your input couldn't be read.");

        let user_input: u32 = match user_input.trim().parse() {
            Result::Ok(value) => value,
            Result::Err(_) => {
                println!("Please enter a number. You entered '{}'.", user_input);
                continue;
            }
        };

        match user_input.cmp(&random_number) {
            Ordering::Less => println!("enter a higher number"),
            Ordering::Greater => println!("enter a smaller number"),
            Ordering::Equal => {
                println!("You win. Congratulations!");
                break;
            }
        }
    }
}
