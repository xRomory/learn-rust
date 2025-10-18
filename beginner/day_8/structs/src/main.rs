/*
 * Struct
 * Structs are similar to tuples in that they can
 * hold multiple values of different types.
 * However, unlike tuples, structs are named
 * and can have methods associated with them.
 *
 * In a struct you'll name each piece of data so
 * it's clear what values mean. Adding these names
 * means that struct are more flexible than tuples.
 *
 * To use a struct after defining it, you create an instance
 * of that struct by specifying concrete values for each field.
 *
 * Like variables, by default, structs are immutable.
 * To make a struct mutable, you use the mut keyword
 * when you create an instance of the struct.
 *
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/*
 * Tuple Structs
 * Don't have names associated with their fields;
 * rather, they have the types of the fields only.
 * Tuple structs are useful when you want to give
 * the whole tuple a name and make the tuple
 * a different type from other tuples, and when naming
 * each field as in a regular struct would be verbose or redundant.
 *
 * Tuple structs require you to name the type of the struct
 * when you destructure them.
*/

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/*
 * Unit-Like Structs
 * Structs can also be define without any fields.
 * They behave similarly with `()`.
 * Can be useful when you need to implement a trait
 * on some type but don't have any data that you want to
 * store in the type itself.
*/

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("romory"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
    };

    // To get a specific value from the struct, use dot notation.
    println!("{:?}", user1.username);

    let mut user2 = User {
        active: false,
        username: String::from("Esereth04"),
        email: String::from("another@email.com"),
        sign_in_count: 0,
    };

    user2.email = String::from("mutable@email.com");

    println!("{:?}", user2.email);

    // Creating Instances from Other Instances
    // with Struct Update Syntax
    let user3 = User {
        // Instead of this:
        // active: user1.active,
        // username: user1.username,
        // email: String::from("instance@email.com"),
        // sign_in_count: user1.sign_in_count,

        // May update shorthand syntax siya
        email: String::from("updatesyntax@email.com"),
        ..user1
    };

    println!("{:?}, {:?}", user3.active, user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

// Yung same name sa parameters, we can use field init shorthand
// so it behaves exactly the same but doesn't have
// the repetition of username and email.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
