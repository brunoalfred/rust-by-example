extern crate rand;


use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    println!("Please enter the Guess...");

    let mut guess = String::new();


    io::stdin().read_line(&mut guess)
        .expect("Failed to read the line!");

    let guess = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);


    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too small!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Less => println!("Too small"),
    }
}