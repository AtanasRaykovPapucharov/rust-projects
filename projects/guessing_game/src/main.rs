use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut counter = 0;

    loop { 
        println!("Please input your guess.");
        let mut guess = String::new();
        counter += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");
        // println!("The secret number is: {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            },
            Ordering::Greater => {
                println!("Too big!");
            },
            Ordering::Equal => {
                println!("You tried {counter} times and you win!");
                break;
            }
        }
    }
}