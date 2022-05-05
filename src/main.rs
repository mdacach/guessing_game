use rand::Rng; // Library from crates.io with RNG functionality

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Inclusive range (..=)

    loop {
        println!("Please input your guess.");

        let guess = {
            let mut input = String::new();

            std::io::stdin()
                .read_line(&mut input) // Will *add* the input to `guess`, not overwrite
                .expect("Failed to read line"); // Panics if there's an error. Similar to `unwrap()`, but with an error message

            // Convert to u32
            let guess: u32 = {
                let parsed = input.trim().parse();
                match parsed {
                    Ok(num) => num,
                    Err(_) => continue,
                }
            };
            guess
        };

        println!("You guessed: {}", guess);

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
