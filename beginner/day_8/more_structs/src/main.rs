/*
 * Example Program Using Structs in Rust
 *
 * In this lesson, we would learn how to add
 * useful functionality with Derived Traits.
 *
 * To printout debugging information about our structs,
 * we can use the Debug trait. This trait enables
 * us to print our structs using the {:?} formatter.
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");

    // To make the print more pretty, we can use `:#?`
    println!("rect1 is {rect1:#?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// This is okay, but not clear in the program
// that the paremeters are related to a rectangle.
// It would be more readable and more manageable
// to group width and height together.

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Refactoring with Tuples
// This is better, but still less clear: tuples don't name
// their elements, so we have to index into the parts of the
// tuple, making the calculation less obvious.
// In the long run, especially as your code grows more complex,
// we have to keep in mind that width is the tuple index 0
// and height is the tuple index 1.

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }
