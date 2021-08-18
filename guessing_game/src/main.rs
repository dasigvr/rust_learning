use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let attempts = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); // crash the program if the line was not read

        // changing 'guess' type
        // In other languages shadowing is often a smell
        // but in rust is safe
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // use shadowing instead of mutable 'attempts'
        // this is nice because after this increment
        // the variable is again immutable
        let attempts = attempts + 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win. You guessed in {} attempts", attempts);
                break;
            }
        }
    }
}
