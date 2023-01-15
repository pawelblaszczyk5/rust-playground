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

// Number of People in the Bus
fn number(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops
        .iter()
        .fold(0, |current_sum, (entered_amount, left_amount)| -> i32 {
            current_sum + entered_amount - left_amount
        })
}

// Reversed Words
fn reverse_words(words: &str) -> String {
    words
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    assert_eq!(past(0, 1, 1), 61000);
    assert_eq!(number(&[(10, 0), (3, 5), (5, 8)]), 5);
    assert_eq!(reverse_words("hello world!"), "world! hello");
}
