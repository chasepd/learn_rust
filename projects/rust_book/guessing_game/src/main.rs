use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut incorrect_guesses = 0;
    let mut non_numeric_guesses = 0;
    loop {
	    println!("What is your guess?");

	    let mut guess = String::new();

	    io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read your guess. Something went wrong.");
	    let guess: u32 = match guess.trim().parse() {
	    	Ok(num) => num,
		Err(_) => {
			non_numeric_guesses = non_numeric_guesses + 1;
			match non_numeric_guesses {				
				1 => println!("Yeah, that's not a number, buddy."),
				2 => println!("That's not a number either."),
				3 => println!("I see you're messing with me."),
				4 => println!("Watch it, buddy."),
				5 => println!("Are you insane?"),
				_ => println!("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
			}
			continue;
		}
            };    

	    println!("You guessed: {}", guess);
	    match guess.cmp(&secret_number) {
	        Ordering::Less => {
			println!("Your guess was too low.");			
			incorrect_guesses = incorrect_guesses + 1;
		},
		Ordering::Greater => { 
			println!("Your guess was too high.");
			incorrect_guesses = incorrect_guesses + 1;
		},
		Ordering::Equal => {
			println!("Ding ding ding! We have a winner!");
			break;
		}
	     };
	     if incorrect_guesses > 5 {
		     println!("https://www.youtube.com/watch?v=bKgWm5TNeBA");
	     }
    }
}
