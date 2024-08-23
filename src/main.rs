use std::io;

// Utils
mod utils;
use crate::utils::clear_screen::clear_screen;
use crate::utils::wait_for::wait_for;

// Games
mod games;
use crate::games::guess_the_number::guess_the_number;
use crate::games::rock_paper_scissors::rock_paper_scissors;

fn main() {
    clear_screen();
    display_choices();
}

fn display_choices() {
    println!("Select your game (first number)");
    let choices = vec!["1. Guess the number", "2. Rock paper scissors"];
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
            2 => {
                println!("You selected 'Rock Paper Scissors'.");
                rock_paper_scissors();
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
                wait_for(2);
                clear_screen();
                display_choices();
            }
        }
    }
}
