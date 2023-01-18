extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    let increase: u32 = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        // input 값을 입력하기 위해 누른 enter로 인해 개행 문자가 발생하여 trim이 꼭 필요함.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type number!");
                continue;
            },
        };

        let guess: u32 = guess + increase;

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!!"),
            Ordering::Greater   => println!("Too big!!"),
            Ordering::Equal     => {
                println!("you win!");
                break;
            },
        }
    }
}
