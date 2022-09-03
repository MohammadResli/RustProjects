use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess-Game-01\n In this Game there is a hidden number and you have to guess it.");

    let hidden_number: i32 = rand::thread_rng().gen_range(1..=100);
    
    // score is calculated based on the fomula score = max(0,101-number of guesses);
    let mut score :i32;
    let mut number_of_guesses: i32 = 0;

    loop {
        println!("Please Enter your guess:");

        number_of_guesses += 1;
        
        score = std::cmp::max(0,101-number_of_guesses);

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
        if number_of_guesses%10==1{
            suffix = "st";
        }
        if number_of_guesses%10==2{
            suffix = "nd";
        }
        if number_of_guesses%10==3{
            suffix = "rd";
        }

        println!("Your {number_of_guesses}{suffix} guessed-number is: {guess}");

        match guess.cmp(&hidden_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!\nYour score is: {score}");
                break;
            }
        }
    }
}
