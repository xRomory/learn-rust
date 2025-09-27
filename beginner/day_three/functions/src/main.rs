/*
  Same sa other PL's functions
    * Pwede ka maglagay ng parameter 
      and each parameter must have types

    * Add return type
*/

/*
  Statements and Expressions
  * Statements: Instructions that perform some actions
    and do not return a value.
  
  * Expressions: Evaluate to a resultant value.
    * Do not include ending semicolons. If a semicolon
      is added at the end of an expression, it turns to a statement.
*/

fn main() {
  basic_function("Hello");

  print_labeled_measurement(300, 'm');

  let x = addtion(7, 10);
  println!("Addition function: {x}");
}

fn basic_function(x: &str) {
  println!(
    "Simple function with parameter string data type: {x}"
  );
}

fn print_labeled_measurement(value: i32, unit: char) {
  println!("The measurement is: {value}{unit}");
}

fn addtion(x: i32, y:i32) -> i32 {
  x + y
}