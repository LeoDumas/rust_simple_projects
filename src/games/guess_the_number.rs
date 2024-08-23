pub fn guess_the_number() {
    use rand::Rng;
    use std::io;

    // Select random number between 1 and 100
    let number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    let mut tries: i32 = 0;
    loop {
        print!("Enter the number: ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_as_int: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if guess_as_int < number {
            println!("Nice try, but bigger!");
            tries += 1;
        } else if guess_as_int > number {
            println!("Nice try, but smaller!");
            tries += 1;
        } else {
            println!(
                "You guessed it, good job! The number was {} in {} tries",
                number, tries
            );
            break;
        }
    }
}
