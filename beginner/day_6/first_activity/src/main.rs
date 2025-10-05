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

#[derive(Debug, Clone)]
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
            2 => display_expenses(&expense),
            3 => transaction_history(&expense),
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
        println!();
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

        println!("\n=== Expense Summary ===");
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

fn display_expenses(expenses: &Vec<Expenses>) {
    if expenses.is_empty() {
        println!("\nNo expenses recorded yet.");
        return;
    }

    println!("\n=== All Expenses ===");
    for expense in expenses {
        print_expense(expense);
    }
    show_total_expenses(expenses);
}

fn transaction_history(expenses: &Vec<Expenses>) {
    if expenses.is_empty() {
        println!("\nNo transaction recorded yet");
        return;
    }

    println!("\nTransaction History Options:");
    println!("1. Show All Expenses");
    println!("2. Sort by date (latest first)");
    println!("3. Sort by date (oldest first)");
    println!("4. Filter by category");
    println!("Choose between (1-4): ");

    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    let option = user_input.trim();

    match option {
        "1" => {
            println!("\n=== All Expenses ===");
            for expense in expenses {
                print_expense(expense);
            }
            show_total_expenses(expenses);
        }

        "2" => {
            let mut date_sorted = expenses.clone();
            date_sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            println!("\n=== Expenses (Latest First) ===");
            for expense in &date_sorted {
                print_expense(expense);
            }
            show_total_expenses(expenses);
        }

        "3" => {
            let mut date_sorted = expenses.clone();
            date_sorted.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            println!("\n=== Expenses (Oldest First) ===");
            for expense in &date_sorted {
                print_expense(expense);
            }
            show_total_expenses(expenses);
        }

        "4" => {
            println!("\nEnter category to filter (ex. Food):");
            let mut category_sort: String = String::new();
            io::stdin()
                .read_line(&mut category_sort)
                .expect("Failed to read line.");

            let category_sort = category_sort.trim();
            let filtered: Vec<&Expenses> = expenses
                                           .iter()
                                           .filter(|e| e.category.trim().eq_ignore_ascii_case(category_sort))
                                           .collect();
            
            if filtered.is_empty() {
                println!("No transaction found for category '{}'", category_sort);
            } else {
                println!("\n=== Expenses in '{}' ===", category_sort);
                for expense in &filtered {
                    print_expense(expense);
                }

                let total: f64 = filtered.iter().map(|e| e.price).sum();
                println!("\nTotal expense for '{}': ₱{:.2}", category_sort, total);
            }
        }
        _ => println!("Invalid option."),
    }
}

fn print_expense(expense: &Expenses) {
    println!(
        "ID: {}, \nItem: {}, \nPrice {}, \nCategory: {}, \nTimestamp: {}\n",
        expense.id, expense.item_name, expense.price, expense.category, expense.timestamp
    );
}

fn show_total_expenses(expenses: &Vec<Expenses>) {
    let total: f64 = expenses.iter().map(|e| e.price).sum();
    println!("\nTotal expense: ₱{:.2}", total);
}