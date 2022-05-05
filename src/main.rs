use rand::Rng; // Library from crates.io with RNG functionality

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Inclusive range (..=)

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        let size_in_bytes = std::io::stdin()
            .read_line(&mut guess) // Will *add* the input to `guess`, not overwrite
            .expect("Failed to read line"); // Panics if there's an error. Similar to `unwrap()`, but with an error message

        // Shadowing guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => continue,
        };
        // From here on, guess is this new value. Not sure if I like this pattern

        println!("You guessed: {}", guess);
        println!("Size (bytes) of input: {}", size_in_bytes);

        use std::cmp::Ordering;
        // Note that Rust has now inferred that secret_number is meant to be u32 too!
        // (In order for the compare to be of the same type)
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
