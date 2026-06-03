The stack holds data associated with a specific function, while the heap holds data that can outlive a function

Rust provides a construct called Box for putting data on the heap. For example, we can wrap the million-element array in Box::new
let a = Box::new([0; 1_000_000]);


Box deallocation principle (almost correct): If a variable is bound to a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

let b = a moves ownership of the box from a to b. Given these concepts, Rust’s policy for freeing boxes is more accurately described as:

Box deallocation principle (fully correct): If a variable owns a box, when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.

The expression &m1 uses the ampersand operator to create a reference to (or “borrow”) m1. The type of the greet parameter g1 is changed to &String, meaning “a reference to a String”

the issue is that the vector v is both aliased (by the reference num) and mutated (by the operation v.push(4)). So to avoid these kinds of issues, Rust follows a basic principle:

Pointer Safety Principle: data should never be aliased and mutated at the same time.

If you write num = &v[2], then v cannot be mutated or dropped while num is in use. But that doesn’t mean it’s invalid to use num again