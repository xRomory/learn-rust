fn main() {
  /*
    Integer Type
    Signed 
      * (same sa Java integer data types: byte, short, ...)
      * Able to use Negative values
      * types: i8, i16, i32, i64, i128, isize
    
    Unsigned:
      u8, u16, u32, u64, u128, usize
    
    One of the things pala na mayroon sa Rust is yung `_`
    as visual separator.
  */

  let string_to_number: u32 = "42"
    .parse()
    .expect("Not a number!"); //Parses a string to integer

  println!("{string_to_number}");

  let visual_separator_sample: i16 = 1_000; // same with 1000
  println!("\n1000 is the same with {visual_separator_sample}");

  let negative_numbers: i8 = -17;
  println!("\nUnsigned number sample: {negative_numbers}");

  /*
    Floating-Point Types
    We have two primitive types sa floating (Stable Rust):
      * f32, and f64

    Upon researching, may f16, and f128 na but unstable feature pa.
    
    by Default, naka f64 kasi on modern CPUs,
    roughly same speed lang sa f32 pero mas precise lang siya,
    just like double
  */

  let x = 2.0; // f64
  let y: f32 = 3.0;

  println!("f64: {x} \nf32: {y}");

  /*
    Numeric Operations:

    Just like on other programming languages,
    you can perform arithmetic operations on Rust.
  */

  let sum = 5 + 10; // Addition
  let difference = 95.5 - 3.2; // Subtraction
  let product = 8.5 * 2.0; // Multiplication

  // Division
  let quotient = 1 / 2;
  let truncated = -6 / 2;

  let remainder = 40 % 5;

  println!()

}
