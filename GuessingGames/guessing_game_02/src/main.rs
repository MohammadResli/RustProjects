use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Guess-Game-02\n In this Game there is a hidden number and you have to guess it.");
    println!("For each guess query you will give us an number,");
    println!(
        " and we will tell you if the hidden number is divisible by your guessed number or not."
    );
    println!("In case your guessed number is the hidden number you will win, and the game will terminate.");

    println!("There are three levels available for this game: ");
    println!("-Easy: the number of allowed guesses to 100.");
    println!("-Medium: the number of allowed guesses to 50.");
    println!("-Hard: the number of allowed guesses to 20.");
    println!("Default level is: Medium (50 allowed guesses).");

    println!(
        "Please Enter the desired level for the game: [E for Easy, M for Medium, H for Hard]."
    );
    let mut level = String::new();
    io::stdin()
        .read_line(&mut level)
        .expect("Failed to read line!");

    let level = level.trim();
    println!("Level: {}", level);
    let allowed_guesses: i32 = match level {
        "E" => 100,
        "M" => 50,
        "H" => 20,
        _ => 50,
    };

    println!("The number of allowed guesses for this game is: {allowed_guesses}");

    let hidden_number: i32 = rand::thread_rng().gen_range(1..=100);

    // score is calculated based on the fomula score = max(0,101-number of guesses);
    let mut score: i32;
    let mut number_of_guesses: i32 = 0;

    loop {
        if number_of_guesses >= allowed_guesses {
            println!("You Lose!! because you don't have any allowed guesses any more.");
            break;
        }
        println!("Please Enter your guess:");

        number_of_guesses += 1;

        score = std::cmp::max(0, 101 - number_of_guesses);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a valid number");
                continue;
            }
        };
        let mut suffix = "th";
        if number_of_guesses % 10 == 1 {
            suffix = "st";
        }
        if number_of_guesses % 10 == 2 {
            suffix = "nd";
        }
        if number_of_guesses % 10 == 3 {
            suffix = "rd";
        }

        println!("Your {number_of_guesses}{suffix} guessed-number is: {guess}");

        if hidden_number == guess {
            println!("You win!!\nYour score is: {score}");
            break;
        }
        if hidden_number % guess == 0 {
            println!("Good, you are almost there.\nThe hidden number is divisible by your guess.");
        } else {
            println!("Sad, the luck isn't on your side this time.");
            println!("\nThe hidden number is NOT divisible by your guess.");
        }
    }
}
