use rand::Rng;
use std::{cmp::Ordering, io};

fn guessing_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error while retriving user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The number is too small"),
            Ordering::Equal => {
                println!("You guessed correctly");
                break;
            }
            Ordering::Greater => println!("The number is too big"),
        }
    }
}

fn main() {
    // guessing_number()
}

fn playground() {}
