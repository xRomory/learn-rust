use std::io::{self, Write};

pub fn try_again() -> Result<bool, io::Error> {
    loop {
        print!("Do you want to try again (yes/y or no/n)?: ");
        io::stdout().flush()?;
        let mut try_input = String::new();
        io::stdin().read_line(&mut try_input)?;

        match try_input.trim().to_lowercase().as_str() {
            "yes" | "y" => return Ok(true),
            "no" | "n" => {
                println!("Round Robin Algorithm Simulation Ended. Goodbye!");
                return Ok(false);
            }
            _ => {
                println!("Please choose between 'yes/y' or 'no/n'.");
                continue;
            }
        }
    }
}
