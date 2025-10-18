/*
    String Slices
    * A reference to a contiguous sequence of the elements of a String.
    * When you use `slice` we can create a range within brackets by specifying
      [starting_index..ending_index],
      * starting_index: first position of the slice
      * ending_index: one more than the last position in the slice.

    * In Rust, `..` is called a range syntax. If you want to start at index 0,
    you can drop the value before the two periods.

    * If your slice includes the last byte of the String, you can drop the trailing
    number.
*/

fn main() {
    let s: String = String::from("String Slices");

    let string = &s[0..6];
    let slices = &s[0..13];

    println!("{string}, {slices}");

    // We can do this as well (see notes)
    let slices = &s[..13];
    println!("{slices}");

    // [0..6] || [..6] is the same

    // Or this (see 4th bullet):
    let len = s.len();

    let slice = &s[3..len];
    println!("{slice}");

    let slice = &s[3..]; // Will output the same with slice variable above.
    println!("{slice}");

    // [3..len] and [3..] are the same

    // With slicing, pag gusto mo from start to end, you can do this: [..]
    let slice_sample: String = String::from("ereT");

    let slice2 = &slice_sample[..];
    println!("{slice2}");

    // Other slices
    // General Slices Type
    let a = [6, 1, 7, 2, 3];

    let arr_slice = &a[..];
    let arr_slice2 = &a[1..3];
    println!("{:?}", arr_slice);
    println!("{:?}", arr_slice2);
    assert_eq!(arr_slice, &[1, 3]);
}
