/*
    The println! macro inserts a newline character after printing your message, ensuring subsequent output appears on a new line.
    The print! macro displays your message without any additional newline characters.

    Placeholders within curly braces {} act as locations for inserting variables or expressions. 
    By assigning values to variables and subsequently referencing them within the println! macro using the {} placeholders, you can create dynamic messages.
    println!("My name is {} and I am {} years old", name, age);

    Rust leverages the Display and Debug traits to control how data appears in console output.
    When Rust encounters a placeholder, it looks for an implementation of the Display trait for the corresponding data type. 
    Think of traits as a way for Rust to define behaviour, like how to print a specific type of data. 
    Rust automatically provides Display implementations for basic types like integers, floats, and strings.

    For more complex data structures (such as collections like tuples or arrays), Rust cannot directly infer how to display them.
    let arr: [u8;5] = [0;5];
    println!("{}", arr);
    Rust compiler will point out an error: "[u8; 5] cannot be formatted with the default formatter".
    The compiler suggests "the trait std::fmt::Display is not implemented for [u8; 5], in format strings you may be able to use {:?} (or {:#?} for pretty-print) instead".
    replace {} with {:?} and recompile. This will print the array content.

    Display trait (fmt::Display) {} for basic data types like integers, floats, and strings etc.
    Debug trait (fmt::Debug) {:?} for complex data structures like tuples or arrays etc.

    Many types in Rust can automatically derive a Debug implementation using #[derive(Debug)]. 
    To print in Debug format, use the {:?} placeholder, or use {:#?} for a more structured, pretty-printed output.

    We can also implement the Display trait for our custom data types.

    Rust offers a variety of formatting placeholders, like {:x} (hexadecimal), {:b} (binary), and {:o} (octal).

    For precision control over floating-point numbers, Rust allows you to specify the desired number of decimal places using the .precision modifier within the println! macro like {:.3}.

    {:>10} Pad with spaces on the left
    {:<10} Pad with spaces on the right
    {:^10} Pad with spaces on both sides

*/
fn main() {
    let name = "Muhammad";
    let age = 25;

    println!("newline");
    print!("same line");
    println!("My name is {} and I am {} years old", name, age);

    let arr:[u8;5] = [0;5];
    /* Debug trait (fmt::Debug) {:?} */
    println!("{:?}", arr); 
    /* Debug trait (fmt::Debug) {:#} */
    println!("{:#?}", arr); 
    
    /* Display trait (fmt::Display) {} */
    println!("Number: {}", 42);
    /* Display trait (fmt::Display) {:hexadecimal}, {:binary}, {:octal},*/
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    /* Display trait (fmt::Display) {:.precision} */
    println!("Pi with 3 decimals : {:.3}", 3.14159);
    /* Display trait (fmt::Display) {} */
    println!("{uname} says {greetings}", uname="Ali", greetings="hello");

    /* Display trait (fmt::Display) {:>10} */
    println!("Right Aligned: {:>10}", "hello");
    /* Display trait (fmt::Display) {:<10} */
    println!("Left Aligned: {:<10}", "hello");
    /* Display trait (fmt::Display) {:^10} */
    println!("Center Aligned: {:^10}", "hello");
}
