use std::{cmp::Ordering, io};
use rand::prelude::*;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("Guess the number!");
    println!("To Quit the game, enter 'q' ");

    loop {
        println!("Please enter your guess (between 1 to 100) : ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        if guess.trim() == "q" {
            println!("You have quit the game.");
            break;
        }else {
            println!("You guessed: {guess}");
            match guess.trim().parse::<i32>() {
                Ok(n) => {
                    let guessed_number = n;
                    match guessed_number.cmp(&secret_number) {
                        Ordering::Less => println!("Guess is too low"),
                        Ordering::Greater => println!("Guess is too high"),
                        Ordering::Equal => {
                            println!("Congrats! You have guessed the number");
                            break;
                        }
                    };
                },
                Err(_) => {
                    println!("Enter a valid number between 1 to 100!");
                }
            };
        }
    }
}
