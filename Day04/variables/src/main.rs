// Variables Summary

/* Variables are the containers that store data */

/* use the let keyword to declare a variable in RUST*/
/* let my_variable: data_type = value; */

/* C strongly typed language (must include data type of variables ) */
/* Python loosely typed language (infer data type of variables based upon their usage )*/
/* RUST strongly typed language but also infer data type of variables based upon their usage */
/* However, there are times when it can't, such as with function arguments. In those situations, you'll need to specify the type explicitly.*/


/* Rust's char type is not limited to a single byte. 
    Instead, it represents a single Unicode Scalar Value. 
    This means Rust's characters support the vast range of characters, symbols, and emojis used across different languages and writing systems. 
    Using UTF-8 encoding ensures your Rust programs can handle text from around the world! 
*/

// Pointer size
/* The size of a pointer can vary depending on computer's architecture. For instance:
    On a 32-bit system, pointers typically occupy 4 bytes
    On a 64-bit system, pointers often take up 8 bytes
*/
/* Rust employs usize and isize to abstract away architecture-specific memory details, making code more portable. */
/* They can be equivalent to u32/u64 and i32/i64 depending on whether you're on a 32-bit or 64-bit system. 
    They ensure your code is adaptable to different architectures without needing to explicitly specify the pointer size. 
    This promotes code portability and simplifies memory management tasks.
*/

/* generic function */
use std::mem::size_of;

/* Generics in Rust allows you to write code that can work with multiple different data types without sacrificing type safety. */
/* The Rust compiler sometimes needs extra help figuring out the specific type you want to use with a generic */
/* turbo fish syntax (::<>) is a way to explicitly tell the compiler which concrete type to fill into a generic placeholder. 
    For example, size_of::<bool>  specifies that you want the size of a bool type specifically. 
*/

fn main() {
    let a:u32 = 10;
    let b = 20;

    println!("value of a is {}", a);
    println!("value of b is {}", b);

    println!("Data type sizes in RUST");

    println!("bool: {} bytes", size_of::<bool>());
    println!("char: {} bytes", size_of::<char>());
    println!("i8: {} bytes", size_of::<i8>());
    println!("i16: {} bytes", size_of::<i16>());
    println!("i32: {} bytes", size_of::<i32>());
    println!("i64: {} bytes", size_of::<i64>());
    println!("u8: {} bytes", size_of::<u8>());
    println!("u16: {} bytes", size_of::<u16>());
    println!("u32: {} bytes", size_of::<u32>());
    println!("u64: {} bytes", size_of::<u64>());
    println!("f32: {} bytes", size_of::<f32>());
    println!("f64: {} bytes", size_of::<f64>());
    println!("usize: {} bytes", size_of::<usize>());
    println!("isize: {} bytes", size_of::<isize>());
}
