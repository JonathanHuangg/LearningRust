use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess: ");

        // Variables usually are immutable so 'mut' adds mutability
        let mut guess = String::new(); // new is an associated function of String type, creates new empty string

        io::stdin() // handle user input
            .read_line(&mut guess) // whatever is actually inputted gets appended to variable guess, mutability matters
            .expect("Failed to read line");
        
        // parse returns result() type that has Ok and Err
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
