/*
    Tuples
    Tuples are a simple yet powerful feature that allows us to group together multiple values of different types. 
    They are useful when you want to return multiple values from a function or when you want to group related data together.

    let my_tuple: (i32, f64, String) = (10, 3.14, "Hello".to_string());
    Here, my_tuple is a tuple that holds an integer, a floating-point number, and a string.

    Accessing elements in a tuple is straightforward. We use the dot operator followed by the index of the element.
    my_tuple.0
    my_tuple.1
    my_tuple.2

    Tuple Destructuring
    let (x, y, z) = my_tuple;

    In Rust, once a tuple is created, its size and the types of its elements cannot be changed. 
    This might give the impression that tuples are immutable, but that's not true. 
    Elements of a tuple can be mutated.

*/

fn circle_properties(radius: f64) -> (f64, f64) {
    let area = std::f64::consts::PI * radius.powi(2);
    let circumference = 2.0 * std::f64::consts::PI * radius;
    (area, circumference)
}

fn tuple_test() {
    let mut another_tuple = (1, vec![2, 3, 4]);
    another_tuple.1.push(5);

    println!("Modified tuple {:?}", another_tuple);
}

fn main() {
    let (area, circumference) = circle_properties(5.0);

    println!("Area {}, Circumference {}", area, circumference);

    tuple_test();
}
