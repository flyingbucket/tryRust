use std::cmp::Ordering;
use std::io;

use rand::{Rng, rng};

fn main() {
    println!("This is a number guessing game. Please type in a number");
    let secret_num = rng().random_range(1..=100);
    println!("The secret_num is:{}", secret_num);
    loop {
        let mut guess = String::new();
        println!("\nPlease enter your guess!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Can't read this line!");
        println!("the number you guessed is :{}", guess);
        let guess: u32 = guess.trim().parse().expect("Please type in a number");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
