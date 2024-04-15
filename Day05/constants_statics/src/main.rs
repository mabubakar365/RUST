/* Consts are essentially fixed values known at compile time. 
    They must have a type annotation and are perfect for situations like array sizes or mathematical constants.
    These are type-safe and prevent unexpected behavior. 
*/

/* Statics are similar to const, but they have a fixed memory location throughout the program's execution. 
    This makes them useful for global variables that need to persist, like configuration settings or file handles. 
    However, unlike const, statics can't be used in all contexts (like within functions) due to their global nature.
    Statics can be declared as mutable (using mut).
    Rust discourages this practice and considers it unsafe.
    unsafe keyword is a backdoor for special cases where you need to bypass Rust's usual safety checks.
*/

static mut COUNTER:u32 = 0;

// fn increment_counter() {
//     COUNTER += 1;
// }

fn increment_counter_unsafe() {
    unsafe {COUNTER += 1;} // this is unsafe
}

fn main() {
    // increment_counter();
    increment_counter_unsafe();
    unsafe {println!("value of COUNTER is {}", COUNTER);} // this is unsafe
}
