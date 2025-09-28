/* 
    Note: Please study more about this feature. This what makes Rust unique.
        Balikan mo yung DSA as always, and master it.

    # Ownership in Rust
    Set of rules that govern how a Rust program manages memory.
    All programs have to manage the way they use a computer's
    memory while running.

    In Rust, memory is managed through a system of ownership
    with a set of rules that the compiler checks. If any of the rules
    are violated, the program won't compile.

    Ownership Rules:
    * Each value in Rust has an "owner".
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
*/

fn main() {

    // Variable scope
    // If variable `s` is called outside the scope, it will become invalid
    {
        // String can be mutated while string literals are immutable.
        let mut s = String::from("hello");

        s.push_str(", world!"); // `push_str()` appends a literal to a String
        println!("{s}"); // Prints hello, world!

        // When we do this:
        // Closer to assignment + resource cleanup
        // This is `drop()` automatically after reassignment
        let mut s2 = String::from("Hi");

        println!("s2 = {s2}");

        s2 = String::from("heal");

        println!("{s2} the world");

        // Variables and Data Interacting with Clone
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");

        /*
            Read more about stack and heap memory allocation,
            Ownership as well
        */

    } // scope ends here -> `drop` is automatically called.

    let s = String::from("Hello po");
    
    takes_ownership(s); // s's value moves into function...
                                    // ... and so is no longer valid here

    let x = 5; // comes into scope

    makes_copy(x); // i32 implements the `Copy` trait
                                // x does NOT move into the function,
                                // so it's okay to use `x` afterward.
    
    /*
        Return Values and Scope
        * Returning values can also transfer ownership.
    */

    // `gives_ownership()` moves its return value to s1
    let s1 = gives_ownership();
    println!("{s1}");

    let s2 = String::from("another");

    // s2 moved to `takes_and_gives_back()`, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);

    println!("{s3}");

}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // scope ends here -> `drop`is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // scope ends here -> Nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}