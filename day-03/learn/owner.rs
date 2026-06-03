fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// At L1, the string “Ferris” has been allocated on the heap. It is owned by first.
// At L2, the function add_suffix(first) has been called. This moves ownership of the string from first to name. The string data is not copied, but the pointer to the data is copied.
// At L3, the function name.push_str(" Jr.") resizes the string’s heap allocation. This does three things. First, it creates a new larger allocation. Second, it writes “Ferris Jr.” into the new allocation. Third, it frees the original heap memory. first now points to deallocated memory.
// At L4, the frame for add_suffix is gone. This function returned name, transferring ownership of the string to full.