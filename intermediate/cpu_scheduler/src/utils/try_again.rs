use std::io::{self, Write};

pub fn try_again() -> Result<bool, std::io::Error> {
    loop {
        print!("\nDo you want to try again? (yes/y or no/n): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return Ok(true),
            "no" | "n" => {
                println!("Simulation Ended.\n");
                return Ok(false);
            }
            _ => {
                println!("Invalid input. Please enter 'y' for yes or 'n' for no.");
                continue;
            }
        }
    }
}
