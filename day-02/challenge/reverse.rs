// Write a function that takes a String, uppercases it, and returns it. Then deliberately use the
// original variable after passing it in, see the error, and fix it. Separately, reverse a Vec<i32>

use std::io;

// fn upper(mut text:str) -> str {
//     for character in text {
//         if 'A' > character {
//             character = character - 'A';
//         }
//     }
// }

fn main() {
    let mut letter = String::new();
    io::stdin().read_line(&mut letter).expect("Failed taking input");

    let c = letter.to_uppercase();
    println!("{c}");

    let a = [1,2,3,4,5];

    for i in (0..5).rev() {
        println!("{}",a[i]);
    }
}