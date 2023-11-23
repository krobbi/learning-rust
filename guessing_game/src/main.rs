use rand::Rng; // Used traits must be imported even if they aren't used by name.
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // Loop gives you an infinite (or indeterminate) loop.
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        // Stdin.read_line reads into an existing mutable String reference.
        // Expect is like unwrap but with a specific error message.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Shadowing is useful for converting type or mutability when we no
        // longer need the original variable.
        // Trim removes surrounding whitespace.
        // Parse can produce any type that can be parsed from a string, so we
        // need an explicit type somewhere.
        // Match can be used on a result enum to handle errors more gracefully.
        let guess: u32 = match guess.trim().parse() {
            // Many traditional 'statements' in Rust are expressions that can
            // return a value.
            Ok(num) => num,
            
            // The err variant has a value, but we don't care about it. '_' is typically used for unused.
            Err(_) => continue, // Jump to the start of the loop.
        };
        
        // This explicit type plus the following comparison has now made the
        // secret number unsigned! I wasn't expecting the type inference to be
        // this clever!
        
        // Macros can handle string formatting, but this is not a language
        // feature.
        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Break out of the loop.
            }
        }
    }
}
