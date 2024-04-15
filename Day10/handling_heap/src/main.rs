/*
    Handling Heap

    Managing Heap is a critical aspect of any programming language. It involves allocating and deallocating memory dynamically during the execution of a program.
    Languages typically adopt one of two common strategies for managing heap memory: 
    Manual control, as seen in C/C++
    Automatic management using a garbage collector, as done by Python.

    Shallow vs Deep copy

    A shallow copy creates a new object and inserts references to the elements of the original object.
    If you modify the original object, the changes will be reflected in the shallow copy.
    Shallow copying is faster and uses less memory because it only copies the references to objects, not the objects themselves.
    Since the copied variables point to the same memory location, changes to the original object will affect the copied object and vice versa. 
    This can lead to unexpected behavior if not handled carefully.

    A deep copy, on the other hand, creates a new object and recursively adds copies of the elements of the original object.
    Changes to the original object do not affect the deep copy. 
    This is useful when you need to work with a copy of an object without affecting the original.
    Deep copying is slower and uses more memory than shallow copying because it involves creating copies of all elements of the copied object. 
    Implementing deep copy can be complex, especially for objects with nested or recursive data structures. 
    You need to recursively make a copy of all the members of the structures.

    Garbage Collection

     The garbage collector, a part of Python’s standard library, is engineered to reclaim memory that’s occupied by objects no longer in use by the program.
     The garbage collector in Python operates using a method called reference counting. 
     Each object has a count that keeps track of the number of references to it. 
     When an object’s reference count falls to zero, it becomes unreachable and is marked for garbage collection.
     Python runs its garbage collector periodically. 
     When the garbage collector encounters an object with a reference count of zero, it frees up its memory.

     Cons of Garbage Collection
      CPU cycles wastage
      memory not freed up immediately

    Manual Control
    
     Dynamic memory management in C is manually controlled by the programmer.
     The functions malloc and free are used for this purpose.
     
     malloc is a function that allocates a specified amount of memory and returns a pointer to the beginning of the allocated block. 
     int *p = malloc(sizeof(int) * 10);  // Allocates memory for 10 integers
     malloc allocates enough memory to hold 10 integers and returns a pointer to the first byte of this memory. The pointer p can now be used like an array.

     free is a function that deallocates memory previously allocated by malloc.
     free(p);  // Deallocates the memory
     free deallocates the memory that p points to. After this line, p should not be used as it is now a dangling pointer.

     Cons of manual control
      Memory leaks
      Dangling pointers
      
*/
fn main() {
    println!("Handling Heap");
}
