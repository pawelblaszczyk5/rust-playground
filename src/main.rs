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

// Beginner Series #2 Clock
fn past(h: i32, m: i32, s: i32) -> i32 {
    h * 60 * 60 * 1000 + m * 60 * 1000 + s * 1000
}

fn main() {
    assert_eq!(past(0, 1, 1), 61000);
}
