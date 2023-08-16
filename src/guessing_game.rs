use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

pub fn run() {
    println!("Enter a number");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("this is the secret number, {}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num ) => num,
            Err(_)=>continue
        };  

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            }
        }
    }
}
