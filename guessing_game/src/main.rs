use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // the new guess: u32 shadows the previous guess: String
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => 
            { 
                if num > 100 {
                    println!("'{num}' is too big. The secret \
                    number is between 0 and 100"); 
                    continue } 
                else 
                    { num }
            },
            Err(e)  => 
            { 
                println!("{e}"); 
                continue
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break; },
            Ordering::Greater => println!("Too big"),
        }
    }
}
