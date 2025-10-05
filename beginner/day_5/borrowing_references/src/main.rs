/*
  References and Borrowing:

  * Reference:
    * Reference is like pointer. It's an address that can be follow
      to access the data stored at that address; that data is owned
      by some other variable.
    * Unlike pointer, a reference is guaranteed to point to a valid
      value of a particular type for the life of that reference.
    * `&` represents references, and they allow you to refer to some
      value without taking ownership of it.
    * The opposite of `referencing` is `dereferencing` which is
      accomplished with the derefence operator `*`.
    * Unlike ownership, reference value is not dropped when the reference
      stops being used.

    * Functions with reference parameter:
      * Doesn't return the actual values. Therefore, there's no need
        to return the values in order to give back ownership,
        because we never had ownership.
    
    * Just as variables are immutable by default, so are references.
    * Mutable references have one big restriction:
      * if you have a mutable reference to a value, you can have no
        other references to that value.

  * Dangling Reference:
    * It is pointer that references a location in memory that may have
      been given to someone else--by freeing some memory while
      preserving a pointer to that memory.
    * This happens when a program has a reference to a piece of memory
      that is no longer valid.

  * Borrowing:
    The action of creating a reference `borrowing`.
*/

fn main() {
  let s = String::from("Reference");
  let refer_variable = calculate_length(&s);

  println!("Reference value: {refer_variable}");
}

fn calculate_length(s: &String) -> usize {
  s.len()
} // s goes out of scope, but because s does not have ownership
  // of what it refers to, the String is not dropped.

// if we do something like this:
// This will cause an error since we cannot borrower immutable.
// That's why we should make this as mutable reference
fn change(some_string: &String) {
  // some_string.push_str(", Immutable");
}

// So, we have to make it mutable by doing this:
fn mut_change(some_string: &mut String) {
  some_string.push_str(", Mutable.");
}

// Bullet 8: Reference
/* 
  * This will cause an error: E0499
  * The restriction preventing multiple mutable references
    to the same data at the same time allows for mutation
    but in a very controlled fashion.
  * The benefit of having this restriction is that Rust
    can prevent data races at compile time.
    * Data races cause undefined behavior and can be difficult
      to diagnose and fix when trying to track them down at runtime;
      Rust prevents this problem by refusing to compile code with
      data races
*/
fn mutable_reference() {
  let mut s = String::from("Invalid");

  // let r1 = &mut s;
  // let r2 = &mut s;

  // But we can do something like this:
  {
    let r1 = &mut s;
    println!("Valid: {r1}");
  }

  let r2 = &mut s;
  println!("Valid: {r2}");

  // or something like this:
  let r3 = &s; 
  let r4 = &s; // Valid since it is a different variable
  // let r5 = &mut s; // Big Problem because we cannot borrow `s`
                      // as mutable it is also borrowed as immutable

  println!("{r3}, {r4}");
  
  // BUT, we can do this:
  let r5 = &s;
  let r6 = &s;
  println!("{r5}, and {r6}");

  let r7 = &mut s;
  println!("{r7}");

  // The resaon behind is:
  // Scope of the variable, we tackled that the scope starts from where
  // where it is introduced and continues through the last time it is used.
}

// Sample of Dangling Reference
/*
  What happens here is we're returning a reference to the String, s
  Once s goes out of scope, it is `drop`, so its memory goes away.

  This happens because s is created inside `dangle_reference()`, when
  the code of this function is finishd, s will be deallocated. But we
  tried to return a reference to it. That means this reference would be
  pointing to an invalid String, which is not good because Rust won't let us.
*/
fn dangle_reference() -> &String {
  let s = String::from("Hello");
  
  &s
}

// For the solution, we can do it like this:
// This works because `ownership` is moved out, and nothing is deallocated.
fn no_dangle() -> String {
  let s = String::from("Valid");

  s
}