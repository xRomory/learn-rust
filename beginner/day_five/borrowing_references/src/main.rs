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
  
  * Borrowing:
    * The action of creating a reference `borrowing`.
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