/*
    Personal Expense Tracker (Beginner)

    * The purpose of this project is to apply what I've learned so far.
        * Scope and Variables
        * Data Types (Arithmetic Functions)
        * Loopings
        * Ownership
        * Borrowing and References
    
    * Project Specification:
        * Basic lang talaga, add expenses
        * Display yung expenses niya
        * Transaction history
            * Sorted by date, or category
            * For Category, I should limit it to prevent user input error
*/
use std::io;

#[derive(Debug)]
struct Expenses {
    item_name: String,
    price: f64,
    category: String,
    date: String,
}

fn main() {
    // let mut expense: Vec<Expenses> = Vec::new();

    println!("=== Personal Expense Tracker ===");

    loop {
        println!("\nMain Menu:");
        println!("1. Calculate Expenses");
        println!("2. View Total Expenses (This Month)");
        println!("3. Transaction History");
        println!("4. Exit");
        println!("Choose between (1-4): ");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to Read Line");

        let choice = match user_input.trim().parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Option, please try again.");
                continue;
            },
        };

        match choice {
            1 => add_expense(),
            2 => display_expenses(),
            3 => transaction_history(),
            4 => {
                println!("\nThank you, bye!");
                break;
            },
            _ => println!("\nInvalid number option, try again"),
        }
        // let 
    }
}

fn add_expense() {
    // Implement
}

fn display_expenses() {
    // Logic for displaying
}

fn transaction_history() {
    // Logic here
}