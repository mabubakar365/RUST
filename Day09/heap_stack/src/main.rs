/*
    When the compiler compiles a program, it creates different sections for each object it generates.
    The linker then gathers all these sections based on linker scripts and produces the final binary. 

    Text (Code) Segment: The text segment contains the executable code of the program. It is read-only and holds the instructions for the program.
    Data Segment: The data segment contains initialized global and static variables.
    BSS Segment: The BSS (Block Started by Symbol) segment contains uninitialized global and static variables. The BSS segment is set to zero during program startup.

    The Text, Data, and BSS segments collectively form the static part of the program that contains fixed-sized instructions and data that persists throughout its execution.

    These should be kept in a non-volatile memory to ensure successful execution of code following a power cycle.

    Heap Segment: The heap segment is used for dynamic memory allocation during the program's runtime. 
    It's important to free the allocated memory after it is no longer needed. 
    Over time, repeated memory allocation without freeing memory can cause the program's memory usage to grow unnecessarily leading to poor performance and runtime allocation failures.

    Stack Segment: The stack segment is used for managing function calls, local variables, and function call frames.

    The stack and heap memory share the dynamic memory area of the program.

    The stack typically starts from the end address of the memory and grows downward.
    The heap starts from the end of the BSS segment and grows upwards. 
    The actual memory layout can vary based on the linker file deployed.

    The text, data, and bss sections are static in nature (their addresses are known and remain constant during runtime).
    The stack and heap are dynamic, changing based on the program’s flow.

    The stack is a dynamic entity that expands or contracts based on stack frames.
    A stack frame or function call frame is like a record that is used to manage the execution of a function.
    It contains information about the function's local variables, parameters, and the necessary information to return to the calling function.
    When a function is called, a new stack frame is created and pushed to the stack, and when the function returns, its stack frame is popped from the stack. 
    This is done by moving the stack pointer up.

    A stack frame typically contains the following components:
    1️⃣ Parameters: The parameters of the function, if any, are stored in the stack frame. These are the values passed to the function when it is called.
    2️⃣ Return Address : The address where the control flow should return after the function completes its execution. The return address is pushed onto the stack by the caller before making the function call.
    3️⃣ Frame Pointer (FP): The frame pointer points to the base of the stack frame and provides a fixed reference point to access function parameters and local variables. It is used to navigate the stack frame, even if the stack pointer (SP) changes during the function's execution.
    4️⃣ Local Variables: Any variables declared within the function are allocated space in the stack frame. These are used to store temporary values and data relevant to the function's execution.

    The stack is ideal for defining and allocating memory when the size is known at compile time and is confined to the scope of a function.

    The heap comes into play, when the size isn’t known at compile time Or when you need to allocate memory that can be utilized beyond the scope of a single function.

    The heap, is a region of memory used for dynamic memory allocation. 
    Unlike the stack, the heap does not have automatic memory management. 
    This means that variables created on the heap will remain there until they are manually deallocated.

    Some of the issues associated with heap memory allocation.
    Memory Leaks: If you forget to free memory when you’re done with it, it leads to a memory leak. This is a common bug in programming.
    Dangling Pointers: If you free a block of memory while there are still pointers referencing it, those pointers become dangling pointers. 
        They point to a block of memory that has been freed, leading to undefined behavior.
    Heap Fragmentation: Over time, the heap can become fragmented with blocks of used and unused memory interspersed. This can make it difficult to find a contiguous block of memory to allocate.
*/
fn main() {
    println!("Heap vs Stack");
}
