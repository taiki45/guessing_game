extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range::<i32>(5, 6);

    println!("Guess number!");

    loop {
        println!("Please input your guess:");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "quit" {
            break;
        }
        let guess: i32 = match input.trim().parse() {
            Ok(n) => n,
            Err(msg) => {
                println!("{}", msg);
                continue;
            },
        };

        println!("You guess {}, and secret is {}", guess, number);

        match number.cmp(&guess) {
            Ordering::Less => println!("Too big!"),
            Ordering::Equal => {
                println!("You got the answer!");
                break;
            },
            Ordering::Greater => println!("Too small!"),
        }
    }
}
