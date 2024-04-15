/*
    The Rust compiler won't let implicit type conversion. 
    It complains about a type mismatch because it refuses to implicitly convert from a one type like floating-point type to another type like an integer. 
    While Rust prefers explicit type conversions, it provides the as operator for those intentional changes. 
    The as operator offers a controlled way to convert between compatible numeric types. 

    Rust prioritizes safety, clarity, and developer control.
    It avoids implicit conversions for several reasons. 
    First, implicit conversions can hide subtle bugs, particularly when data precision is lost or values change unexpectedly.
    By making conversions explicit, Rust forces you to confront these potential problems head-on.
    Second, explicit conversions enhance code readability by clearly signifying where type changes are happening and what your intentions are.
*/

// fn test_function() {
//     let PI:f64 = 3.1456;
//     let WHOLE_PI:u32 = PI; // Error! Type mismatch (f64 to i32)
//     println!("Whole part of PI is {}", WHOLE_PI);
// }

fn test_function_valid() {
    let pi:f64 = 3.1456;
    let whole_pi:u32 = pi as u32;
    println!("Whole part of PI is {}", whole_pi);
}

fn main() {
    // test_function();
    test_function_valid();
}
