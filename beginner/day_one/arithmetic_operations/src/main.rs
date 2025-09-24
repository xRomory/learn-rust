fn main() {
  /*
    Take note: 
    in Rust, hindi mo pwede gawin yung reassignment of value
    na nagagawa mo sa other high-level languages such as Java and Python.

    For example:
    let x = ...
    x = ... 

    * This cause can error since when you use `let`, 
    the variable becomes immutable
  */

  let x = 6;
  let y = 12;
  let sum = x + y;

  println!("The value of x: {x}");
  println!("The value of y: {y}");
  println!("x + y = {sum}");

  // Constant variable
  const THREE_HOURS_IN_SECONDS: u32 = 3600 * 3; //There are 3600 seconds in 1hr
  println!("\n{THREE_HOURS_IN_SECONDS}");

  /*
    Shadowing:

    This allows you to declare a new variable 
    with the same name as a previous variable.

    Since it's basically a new variable,
    we can assign a new data type (see ex. 2).

    Things will be different if the variable is mutable.
    It will caused a compile-error since you're basically
    using the same variable (see ex. 3).
  */

  // Ex. 1 
  let x = 5;
  println!("{x}");

  let x = x + 1;

  {
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }

  println!("Value of x: {x}");

  // Ex. 2
  let spaces = "   "; //string type
  let spaces = spaces.len();  //number type

  println!("{spaces}");

  // Ex. 3
  let mut numbers = "123";
  println!("{numbers}");
  
  numbers = 123; // Error: expected &str
}