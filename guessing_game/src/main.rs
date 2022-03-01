use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	
	let console_in = io::stdin();
	let mut rng = rand::thread_rng();
	
	println!("Welcome to the guessing game.");
	
	let random_number: i32 = rng.gen_range(1..=100);
	println!("Your random number is {}.", random_number);
	
    loop {
        println!("Please enter your guess.");
		let mut user_input = String::new();
		console_in
			.read_line(&mut user_input)
			.expect("Your input couldn't be read.");
		
		println!("Your input was: {}", user_input);
		let mut user_input_parsed: i32 = -1; 
		
		match user_input.trim().parse() {
            Result::Ok(value) => {
                if value < 1 || value > 100 {
                    println!("Pleas enter a number between 1 and 100.");
                }
                user_input_parsed = value;
            },
            Result::Err(_) => println!("Please enter a number. You entered '{}'.", user_input)
		};
		
		if user_input_parsed != -1 {
            match user_input_parsed.cmp(&random_number) {
                Ordering::Less => println!("enter a higher number"),
                Ordering::Greater => println!("enter a smaller number"),
                Ordering::Equal => {
                    println!("You win. Congratulations!");
                    break;
                }
            }
		}
	}
	
}