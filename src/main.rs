use rand::Rng; // Library from crates.io with RNG functionality

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // Inclusive range (..=)

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    let size_in_bytes = std::io::stdin()
        .read_line(&mut guess) // Will *add* the input to `guess`, not overwrite
        .expect("Failed to read line"); // Panics if there's an error. Similar to `unwrap()`, but with an error message

    println!("You guessed: {}", guess);
    println!("Size (bytes) of input: {}", size_in_bytes);
}
