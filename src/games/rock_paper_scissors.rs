use rand::seq::SliceRandom;
use std::io;

pub fn rock_paper_scissors() {
    print!("Select number of rounds: ");
    let nb_round = ask_number_of_round();
    println!("Ready for {} rounds!", nb_round);

    // Define possible choices
    let choices = vec!["Rock", "Paper", "Scissors"];
    let mut user_score = 0;
    let mut computer_score = 0;
    let mut current_round = 1;

    while current_round <= nb_round {
        println!("\nRound {} of {}", current_round, nb_round);
        print!("Your choice: ");

        // Get user choice
        let mut choice = String::new();
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();
        if !choices.contains(&choice) {
            println!("Invalid choice, please choose Rock, Paper, or Scissors.");
            continue; // Ask again if invalid
        }

        println!("You chose: {}", choice);

        // Randomly select computer's choice
        let computer_choice = choices.choose(&mut rand::thread_rng()).unwrap();
        println!("Computer chose: {}", computer_choice);

        match determine_winner(choice, computer_choice) {
            Some("user") => {
                println!("You win this round!");
                user_score += 1;
            }
            Some("computer") => {
                println!("Computer wins this round!");
                computer_score += 1;
            }
            _ => {
                println!("This round is a draw!");
            }
        }

        current_round += 1;
    }

    println!(
        "\nFinal Score: You: {}, Computer: {}",
        user_score, computer_score
    );
    if user_score > computer_score {
        println!("Congratulations! You won the game!");
    } else if computer_score > user_score {
        println!("Computer wins the game. Better luck next time!");
    } else {
        println!("The game is a draw!");
    }
}

fn determine_winner<'a>(user_choice: &'a str, computer_choice: &'a str) -> Option<&'a str> {
    match (user_choice, computer_choice) {
        ("Rock", "Scissors") | ("Scissors", "Paper") | ("Paper", "Rock") => Some("user"),
        ("Scissors", "Rock") | ("Paper", "Scissors") | ("Rock", "Paper") => Some("computer"),
        _ => None, // It's a draw
    }
}

fn ask_number_of_round() -> i32 {
    let mut number_of_round = String::new();
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut number_of_round)
        .expect("Failed to read line");

    let number_of_round_as_int: i32 = match number_of_round.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number");
            return ask_number_of_round(); // Recursively ask again if parsing fails
        }
    };
    number_of_round_as_int
}
