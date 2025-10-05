/*
    Control Flow

    * If-Else Expressions
        * Same concept sa other PLs.
        * Same syntax to most PLs
        * `If` is considered an expression.

    * Repetition with Loops
        * `loop`: break, continue
        * `while`: breaks if the conditions were met
        * `for`: same sa Python (I think).
            * Basta `for` gives you more control, and faster than `while`
        
        * All of loop functions naman are okay since it will depends sa scenario
*/

fn main() {
    simple_if_else();

    simple_loop();

    nested_loops();

    while_loop();

    for_loop();
}


fn simple_if_else() {
    let number: i8 = 3;

    // If-else
    if number < 5 {
        println!("Is {number} less than 5: true");
    } else {
        println!("Is {number} greater than 5: false");
    }

    // If
    if number != 0 {println!("The value of number: {number}, is other than zero.")};

    // Else if
    if number % 2 == 0 {
        println!("Even number");
    } else if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else {
        println!("Odd number");
    }

    let condition: bool = true;
    let evaluate = if condition { 5 } else { 6 };

    println!("The value of number is: {evaluate}");
}

fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 { break counter * 2; }
    };

    println!("The result is: {result}");
}

/*
    Loops Labels
    * Sa Rust, when you have loops within loops,
      yung `break` and `continue`apply to innermost
      loop. Pero you can optionally specify a loop label
      on a loop that you can then use with `break` or `continue`.

    * Loop Labels must begin with a single quote (').
*/
fn nested_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");

            if remaining == 9 { break; }

            if count == 2 { break 'counting_up; }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("End of while loop...");

    // We can use `while` construct to loop over collection or array.
    // For this example, this is unsafe kasi pag binawasan ko yung values
    // inside the array but I didn't update index variable, it will cause an error.
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", arr[index]);

        index += 1;
    }
}

fn for_loop() {
    // In this example, using `for` is more safety.
    let arr = [10, 11, 12, 13, 14];

    for element in arr {
        println!("The value is {element}");
    }

    // Example 2:
    for number in (1..4).rev() { // `rev()` is reverse method
        println!("{number}");
    }

    println!("For loop countdown");
}