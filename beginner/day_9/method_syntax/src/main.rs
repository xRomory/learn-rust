/*
 * Method Syntax
 *
 * Methods are similar to functions: we declare them with
 * `fn` keyword and a name, they can take parameters,
 * they can return values, and they contain some code
 * that's run when the method is calle from somewhere else.
 * The difference is that methods are defined within the context
 * of a struct (or an enum or a trait object), and their
 * first parameter is always `self`, which represents the instance
 * of the struct the method is being called on.
 */

/*
 * Where's the -> Operator?
 * In C and C++, two diffrent operators are used for calling methods:
 * `.` if you're calling a method on an object, and `->` if you're
 * calling a method on a pointer to an object. Rust has a feature
 * called "automatic referencing and dereferencing". Calling methods
 * is one of the few places in Rust with this behavior.
 *
 * When you call a method with object.something(), Rust automatically
 * adds `&`, `&mut`, or `*` so object matches the signature of the method.
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
 * To define the function within the context of the struct,
 * we use an `impl` (implementation) block. Everything within the
 * `impl` block will be associated with the struct named after.
 * Then we move the function within the `impl` block and change
 * its first parameter to `&self`, which is a reference to the
 * instance of the struct that the method is being called on.
 *
 * By using `&self`, we indicate that the method borrows
 * the instance immutably, meaning we can read data from it
 * but cannot modify it. If we wanted to modify the instance,
 * we would use `&mut self` instead.
 *
 * All functions defined within an `impl` block are
 * called associated functions because they are associated
 * with the type named after the `impl`.
 *
 * We can define associate functions that don't take `self`
 * as their first parameter. These are often called new,
 * but new isn't a special name in Rust, and isn't built into the language.
 */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We can give a method the same name as one of the struct's fields.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 70,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width());
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // To call associated functions:
    let square = Rectangle::square(3);
    println!("Square value: {:?}", square);
}
