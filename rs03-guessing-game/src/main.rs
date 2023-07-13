use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng()
                                    .gen_range(1, 11);
    println!("Hint: {} :)", secret_number);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        if guess == ":quit".to_string() {
            break;
        }

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess = match guess
                                .trim()
                                .parse::<i32>() {
                                    Ok(num) => {
                                        num
                                    },
                                    Err(_) => {
                                        println!("{}", "Error: You shoull input number".red());
                                        continue
                                    }
                                };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "Your guess is correct!".green());
                break;
            },
            Ordering::Greater => {
                println!("{}", "Your guess is greater than the secret value!".yellow());
            },
            Ordering::Less => {
                println!("{}", "Your guess is les than the secret value!".yellow());
            }
        }
    }
}
