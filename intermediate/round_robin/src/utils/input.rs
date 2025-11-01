use std::io::{self, Write};

pub fn user_input(message: &str) -> Result<String, io::Error> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn valid_input(input: &str) -> Result<i32, &'static str> {
    match input.trim().parse::<i32>() {
        Ok(num) if num >= 0 => Ok(num),
        Ok(_) => Err("Please enter a non-negative integer."),
        Err(_) => Err("Invalid input: Enter a valid integer."),
    }
}
