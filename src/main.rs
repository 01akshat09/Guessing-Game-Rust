use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number is {secret_number}");
    println!("Please input your number: ");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to take user input");

        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a vaild input");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too Big!"),
            Ordering::Less => println!("Too Small!"),
        }
    }
}
