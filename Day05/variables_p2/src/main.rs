// Mutability
/* By default, all variables in Rust are immutable (read-only). 
    This promotes safety and prevents accidental changes. 
    To make a variable mutable (changeable), you need to explicitly use the mut keyword. 
*/
// fn err_mut_func()
// {
//     let x = 5;
//     println!("value of x: {}", x);

//     /* Error: cannot assign twice to immutable variable */
//     x = 6;
//     println!("value of x: {}", x);
// }

// fn err_uninit_func()
// {
//     let x:u32;

//     /* Error: `x` used here but it isn't initialized */
//     println!("value of x: {}", x);


// }

fn mut_func()
{
    let mut x = 5;
    println!("value of x: {}", x);

    x = 6;
    println!("value of x: {}", x);
}

fn main() {
    mut_func();
    // err_mut_func();
    // err_uninit_func();

}
