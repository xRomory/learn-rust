use core::f64;
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
use chrono::{DateTime, Local};

#[derive(Debug)]
struct Expenses {
    id: i32,
    item_name: String,
    price: f64,
    category: String,
    timestamp: String,
}

fn main() {
    let mut expense: Vec<Expenses> = Vec::new();
    let mut next_id: i32 = 1i32;

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
            .expect("Failed to read line.");

        let choice = match user_input.trim().parse::<i8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Option, please try again.");
                continue;
            },
        };

        match choice {
            1 => add_expense(&mut expense, & mut next_id),
            2 => display_expenses(),
            3 => transaction_history(),
            4 => {
                println!("\nGood bye, mabuhay!");
                break;
            },
            _ => println!("\nInvalid number option, try again"),
        }
    }
}

fn add_expense(expenses: &mut Vec<Expenses>, next_id: &mut i32) {
    let mut repeat: bool = true;

    while repeat {
        let now: DateTime<Local> = chrono::Local::now();
        let timestamp_string = now.format("%Y-%m-%d %H:%M:%S").to_string();
        println!("Enter item category:");
        let mut item_category: String = String::new();
        io::stdin()
            .read_line(&mut item_category)
            .expect("Failed to read line.");

        println!("Enter item name:");
        let mut item_name: String = String::new();
        io::stdin()
            .read_line(&mut item_name)
            .expect("Failed to read line.");

        println!("Enter item price:");
        let mut input_price = String::new();
        io::stdin()
            .read_line(&mut input_price)
            .expect("Failed to read line.");

        let price: f64 = match input_price.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid price input, try again.");
                return;
            }
        };

        let expense: Expenses = Expenses {
            id: *next_id as i32,
            item_name: item_name.trim().to_string(),
            price,
            category: item_category.trim().to_string(),
            timestamp: timestamp_string,
        };

        print_expense(&expense);
        expenses.push(expense);
        *next_id += 1;
        println!("Expenses added!");

        println!("Add another item? (y/n)");
        'y_or_n: loop {
            let mut again: String = String::new();
            io::stdin()
                .read_line(&mut again)
                .expect("Failed to read line.");

            let input: String = again.trim().to_lowercase();
            if input == "y" {
                break 'y_or_n;
            } else if input == "n" {
                break repeat = false;
            } else {
                println!("Please choose between 'y' or 'n' only.");
                continue;
            }
        };
    }
}

fn display_expenses() {
    // Logic for displaying
}

fn transaction_history() {
    // Logic here
}

fn print_expense(expense: &Expenses) {
    println!(
        "\n === Expense Summary: ===\nID: {}, \nItem: {}, \nPrice {}, \nCategory: {}, \nTimestamp: {}\n",
        expense.id, expense.item_name, expense.price, expense.category, expense.timestamp
    );
}