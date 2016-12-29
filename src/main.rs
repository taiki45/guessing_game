extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range::<i32>(1, 10);

    println!("Guess number!");

    println!("Please input your guess:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let guess = input.trim();

    println!("You guess {}, and secret is {}", guess, number);

    if guess == number.to_string() {
        println!("Got answer!");
    } else {
        println!("You got wrong!");
    }
}
