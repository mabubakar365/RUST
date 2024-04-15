/*
    Strings are like dynamic arrays of chars.
    They can grow and shrink in size, and they offer a variety of methods for manipulation.

    In Rust, we use the String type to represent strings.
    To create a string in Rust, you can use the from method of the String type from the Rust standard library. 
    When we call this method, the Rust library internally allocates memory on the heap and initializes it with the content passed to the from method.
    let mut my_string = String::from("Rust🦀"); 

    We can have emojis as part of strings.
    Rust's char type is not limited to a single byte. Instead, it represents a single Unicode Scalar Value.
    Similar to Rust chars, Rust strings are not limited to ASCII chars and can represent UTF-8.

    Elements in a Rust string are not all of the same size, and we cannot access them using indexing directly, as rust does not know what the right index boundary should be.
    To access individual elements in Rust, we should access the index using either chars().nth() or bytes().nth(). 
    chars().nth(0) convert the string into an iterator of chars and then select the specific index
    bytes().nth(0) convert the string into iterator of bytes and then select the specific index

    Strings in Memory
    A String internally has three parts
     1. a pointer to the data on the heap
     2. the length of the string (how many bytes)
     3. its capacity (the total allocated space on the heap) 

    Rust's String type explicitly stores the length of the string.
    This eliminates the need for null termination and prevents the risk of reading invalid memory.

    In order to expand the string
     If the current capacity is enough, Rust will simply append the new string.
     However, if it's not, Rust will allocate a new chunk of memory on the heap (likely larger than the original) to accommodate the expanded string and then copy the content over.
*/

fn access_string_element() {
    let message = String::from("Rust 🦀");

    //Error: `String` cannot be indexed by `{integer}`
    //println!("First element of message is {} ", message[0]);
    println!("Length of message is {} ", message.len());

    // size of first crab 'Rust'
    println!("char: {} size: {}", message.chars().nth(0).unwrap(), message.chars().nth(0).unwrap().len_utf8());

    // size of character Emoji
    println!("char: {} size: {}", message.chars().nth(5).unwrap(), message.chars().nth(5).unwrap().len_utf8());
}

fn main() {
    access_string_element();
}
