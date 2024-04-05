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
