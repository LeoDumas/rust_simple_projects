use std::io;

mod games;
use crate::games::guess_the_number::guess_the_number;

fn main() {
    display_choices();
}

fn display_choices() {
    println!("Select your game (first number)");
    let choices = vec!["1. Guess the number"];
    for choice in &choices {
        println!("{}", choice);
    }

    loop {
        let mut user_choice = String::new();

        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read line");

        let choice_as_int: i32 = match user_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match choice_as_int {
            1 => {
                println!("You selected 'Guess the number'.");
                guess_the_number();
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
