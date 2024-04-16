/*
    fn main() is the heart of most Rust programs, the entry point where the action begins.
    println! is a macro
    Rust uses ! to distinguish macros from regular functions. 
    rustc main.rs tells the Rust compiler (rustc) to compile the RUST code.
    ./main runs the executable

    Cargo: The Build Master
     Rust – Cargo simplifies compiling, managing dependencies, packaging... everything to do with building a project. 
     cargo new hello. This creates a new directory called hello
      Cargo.toml: This is the core of your project. It's where you define your project's name, version, dependencies, and more. 
                  Think of it like the Makefile for your Rust project.
      src/: Your Rust code lives here! You'll find a main.rs file, ready and waiting to be filled with your code.
      .gitignore: This file tells Git which files it shouldn't track.

      cargo build, Cargo will compile your code. 
      cargo run, To run your project.

      Cargo.lock: This file keeps a record of the exact versions of your dependencies. 
                  Think of it as a way to make sure your project builds the same way every time.

      target/ directory: This is where all the compiled files and other build goodies end up.

*/
fn main() {
    println!("Hello, world!");
}
