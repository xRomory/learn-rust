use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
  println!("Guess the number!");

  // Using `rand` library we can use functions such as random_range
  let secret_number = rand::rng().random_range(1..=100);

  println!("The secret number is: {secret_number}");

  println!("Please input your guess.");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  println!("You guessed: {guess}");

  // Will fix later. Not matching an integer
  match guess(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
  }
}