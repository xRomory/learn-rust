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
    }
}