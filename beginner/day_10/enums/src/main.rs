/*
 * Enums in Rust
 *
 * Enums (short for "enumerations") are a powerful feature in Rust
 * that allows you to define a type by enumerating its possible variants.
 * Enums are particularly useful when you want to represent a value
 * that can be one of several different types.
 *
 * Using enums hav
 */

// IP Address Enum Example
// `IpAddrKind` is now a custom data type that we can use elsewhere in our code.
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // Note that the variants are namespaced under its identifier `IpAddrKind`.
    // we use a double colon `::` to separate the two.
    // This is useful because now both values are the same type, `IpAddrKind`.
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Hello, world!");
}
