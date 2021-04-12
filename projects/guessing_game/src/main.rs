use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("DEBUG: Secret number is {}", secret_number);
    loop {
	    println!("What is your guess?");

	    let mut guess = String::new();

	    io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read your guess. Something went wrong.");

	    println!("You guessed: {}", guess);

	    let guess: u32 = guess.trim().parse().expect("Please type a number.");    

	    match guess.cmp(&secret_number) {
	        Ordering::Less => println!("Your guess was too low."),
		Ordering::Greater => println!("Your guess was too high."),
		Ordering::Equal => {
			println!("Ding ding ding! We have a winner!");
			break;
		}
    	}
    }
}
