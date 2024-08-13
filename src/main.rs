use std::path::PathBuf;  // Bring the PathBuf to the namespace with `use`

fn main() {
    println!("Hello, world!");

    let _new_path = PathBuf::new();


    // Declaring Variables
    // let x: i32 = 5; //In this case, x is an integer of 32 bits.
    let _x = 5; // immutable variable, use underscore if the variable isn't being used yet
    let mut _y = 5; // mutable variable
    _y = 6; // this is okay because y is mutable

    // ---- Signed and Unsigned Integers ----
    // non-exhaustive list of signed integer types:
    // isize
    // i64
    // i32
    // i16
    // i8
    // formula for the range of signed integers is -2^(n-1) to 2^(n-1) - 1

    // non-exhaustive list of unsigned integer types
    // usize
    // u64
    // u32
    // u16
    // u8
    // Unsigned integers have a range from 0 to 2^n - 1

    // --- Floating Types ---
    // f32 and f64 represent 32 bit and 64 bit floating point numbers respectively
    // use f64 because it has higher precision
    // A floating point number can be written with decimal points (for example, 3.14), or in scientific notation (such as 5E-10).

    // --- Strings ---
    // Rust has two main types of strings: String and &str
    // String is a growable, mutable, owned, heap-allocated data structure. 
    // It is used when you need to own string data, such as for storing it in structs.

    // str is a string slice and it is usually seen in the form of borrowed string, &str. 
    // It is used for passing around pieces of strings in a read-only fashion.

    // --- Collections ---
    // Slices
    // Slices in Rust allow you to reference a contiguous sequence of elements in a collection rather than the whole collection.
    // Slices are used often with strings, arrays, and vectors.
    // They are defined by a pointer to the first element in the slice and a length.

    let s = String::from("hello world");
    let _hello = &s[0..5]; // hello will be a slice that contains the first 5 characters of s
    let _world = &s[6..11]; // world will be a slice that contains the last 5 characters of s

    // --- Vectors ---
    // They allow you to store multiple values of the same type in a contiguous block of memory
    // Vectors are implemented as a struct called Vec
    // You can create a new, empty vector with Vec::new()
    // To create a vector that is initialized with certain values, you can use the vec!

    let _v = vec![1, 2, 3]; // v is a vector that holds the integers 1, 2, and 3

}
