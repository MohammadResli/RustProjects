use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess-Game-01\n In this Game there is a hidden number and you have to guess it.");

    let hidden_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Enter your guess:");

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

        println!("You guessed {guess}");

        match guess.cmp(&hidden_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
