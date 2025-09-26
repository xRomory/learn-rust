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

    Just like in other programming languages,
    you can perform arithmetic operations on Rust.

    Integer divisions truncates toward zero to the
    nearest integer.
  */

  let sum = 5 + 10; // Addition
  let difference = 95.5 - 3.2; // Subtraction
  let product = 8.5 * 2.0; // Multiplication

  // Division
  let quotient = 1 / 2;
  let truncated = -7 / 2; // Results in -3

  let remainder = 40 % 5;

  println!("Sum: {sum}\nDifference: {difference}\nProduct: {product}\nQuotient: {quotient}");
  println!("Truncated: {truncated}\nRemainder: {remainder}");

  /* 
    Boolean Type

    Same syntax to Python, same fucntions sa other PL
  */

  let t = true;
  let f: bool = false;

  /* 
    Character Type

    * Same practice to Java, char literals nakalagay sa single quotes
    * 4-bytes in size, and represents a Unicode scalar value.
  */

  let char_non_explicit = 'z';
  let char_explicit: char = 'Z';
  let heart_eyed_cat = '😻'; // We can do this since unicode

  /* 
    Compound Type

    There are two primitive compound types: Tuples and Arrays.

    With Tuples, it is considered as a single compound element.
    * Same sa other languages, index counting rin siya
    * If tuple don't have explicit values, it is called: `unit`

    With array, every elements must have the same type.
    Sa Rust, arrays have a fixed length.
  */

  // Tuples
  let tuple: (i32, f32, u8) = (-500, 0.4, 1);

  // We can pass `tuple` like this:
  let (a, b, c) = tuple;

  println!("The value of a: {a}\nThe value of b: {b}\nThe value of c: {c}");

  let negative_five_hundred = tuple.0;
  let point_four = tuple.1;
  let one = tuple.2;

  println!("First index: {negative_five_hundred}");
  println!("Second index: {point_four}");
  println!("Third index: {one}");

  // Arrays
  let arr = [1, 2, 3, 4, 5];
  println!("Array: {:?}", arr); // When printing an array, use `:?` debug format specifier

  // We can also do this if the same values yung elements
  // value: 3, length: 5

  let three_five = [3; 5];
  println!("{:?}", three_five);

  // Same with other PLs when accessing array elements
  let first_value = arr[0];
  let last_value = arr[4];

  println!("First value: {first_value} \nLast value: {last_value}");
}
