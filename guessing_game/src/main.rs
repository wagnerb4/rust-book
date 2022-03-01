use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	
    let mut user_input = String::new();
	let console_in = io::stdin();
	let mut rng = rand::thread_rng();
	
	println!("Welcome to the guessing game.");
	
	let random_number: i32 = rng.gen_range(1..=100);
	println!("Your random number is {}.", random_number);
	
    loop {
		console_in
			.read_line(&mut user_input)
			.expect("Your input couldn't be read.");
		
		let user_result = user_input.trim().parse(); 
		
		let user_input: i32 = match user_input.trim().parse() {
			io::Result::Ok(user_input) => user_input,
			io::Result::Err(_) => println!("Please enter a number. You entered '{}'.", user_input)
		};
		
		println!("Your input was: {}", user_input);
		
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
