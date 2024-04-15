/* variable shadowing in Rust : redeclare a variable with the same name using the let keyword.
    The new declaration shadows the previous one.
    Shadowing creates an entirely new variable in a different memory space.
    The original isn't modified or replaced.
    The shadowing re-binds the variable name to a new memory location.
    The original memory location still exists, and the value it holds is technically still there.
    The important point is that you can no longer access that old value using the same variable name.
*/

fn main() {
    let x = "Hello";

    let y = &x;

    println!("Value of x is {}", x);

    let x = 3.1456;

    println!("New value of x is {}", x);

    println!("Old value of x is {}", *y);
}
