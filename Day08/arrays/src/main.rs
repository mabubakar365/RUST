/*
    Arrays are a fundamental data structure that allow us to store multiple values of the same type in a single variable.
    
    In Rust, arrays are declared using the syntax let variable_name : [data_type; size].
    let my_numbers: [u32; 5] = [1, 2, 3, 4, 5];
    my_numbers is an array that holds five unsigned 32-bit integers. 

    let my_numbers = [0; 5];
    my_array is an array of five integers, all initialized to 0.

    The size of arrays in Rust is fixed at compile time, and each element in the array must be of the same type. 
    This can be different from other datatypes like vector that can be resized dynamically.


    Note that you don't need to specify the array data type and size if you are initializing it right away. Rust can infert it.

    Arrays of different sizes or different element types are considered different types. 
    Therefore, you cannot directly assign an array of one type to an array of another type.

*/

fn print_array_a0() {
    let numbers = [0; 5];

    for number in numbers {
        print!("{} ", number);
    }

    println!();
}

fn print_array_a1() {
    let numbers = [1,2,3,4,5];

    for number in numbers {
        print!("{} ", number);
    }

    println!();
}

// use debug trait 
fn print_array_a2() {
    let numbers = [1,2,3,4,5];

    // :? is debug placeholder
    println!("{:?} ", numbers);
}

fn print_array_a3() {
    let numbers = [1,2,3,4,5];

    // :? is debug placeholder
    print!("{} ", numbers[0]);
    print!("{} ", numbers[1]);
    print!("{} ", numbers[2]);
    print!("{} ", numbers[3]);
    println!("{}", numbers[4]);
}

// fn invalid_index_access() {
//     let mut arr = [0;5];

//     // Error: index out of bounds: the length is 5 but the index is 1000
//     arr[1000] = 100
// }

fn mismatched_type() {
    let mut arr1 = [0;5];
    let arr2 = [10,20,30];

    // Error: expected an array with a fixed size of 5 elements, found one with 3 elements
    arr1 = arr2;
}

fn main() {
    print_array_a0();
    print_array_a1();
    print_array_a2();
    print_array_a3();
    // invalid_index_access();
    // mismatched_type();
}
