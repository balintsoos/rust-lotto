use std::io;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the guess");

    println!("You guessed {guess}")
}
