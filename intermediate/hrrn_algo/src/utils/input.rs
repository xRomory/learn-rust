use std::io::{self, Write};

pub fn user_input(message: &str) -> Result<String, io::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn valid_input(input: &str) -> Result<u32, &'static str> {
    match input.trim().parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input. Please enter a valid positive integer."),
    }
}
