extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range::<i32>(1, 2);

    println!("Guess number!");

    println!("Please input your guess:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let guess: i32 = input.trim().parse()
        .expect("Please type a number!");;

    println!("You guess {}, and secret is {}", guess, number);

    if guess == number {
        println!("Got answer!");
    } else {
        println!("You got wrong!");
    }
}
