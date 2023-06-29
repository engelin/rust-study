use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        // let guess: u32 = guess.trim()
        //                       .parse::<u32>() // .parse() is okay it'll change to u32 since this variable re-declair with u32
        //                       .expect("Please type a number!");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {err}");
                continue;
            },
        };

        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
