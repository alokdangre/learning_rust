// Flatten and deduplicate
// Input:  vec![vec![3,1,2], vec![2,4,1], vec![5]]
// Output: vec![1, 2, 3, 4, 5]  // unique, sorted
// Rules: function takes &[Vec<i32>] — borrow only, no cloning the outer structure. Return a new Vec<i32>.

use std::io;
fn deduplicate(m: &[Vec<i32>]) -> Vec<i32> {
    let mut flat: Vec<i32> = vec![];
    for inner in m {
        for &k in inner {
            println!("{} ", k);
            if !flat.contains(&k) {
                flat.push(k);
            }
        }
    }
    flat
}

fn sort(m: &mut Vec<i32>) {
    let s = m.len();
    for i in 0..s {
        for j in i + 1..s {
            if m[i] > m[j] {
                let t = m[i];
                m[i] = m[j];
                m[j] = t;
            }
        }
    }
}

fn main() {
    let mut mat: Vec<Vec<i32>> = vec![];
    loop {
        let mut inner_vec: Vec<i32> = vec![];
        let mut done = false;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            match trimmed.parse::<i32>() {
                Ok(num) => inner_vec.push(num),
                Err(_) => {
                    if trimmed == "done" { done = true; break; }
                    else if trimmed == "d" { break; }
                    // else: ignore and keep reading
                }
            }
        }
        mat.push(inner_vec);
        if done { break; }
    }
    let n = mat.len();
    for inner in 0..n {
        let m = mat[inner].len();
        for j in 0..m {
            println!("{} ", mat[inner][j]);
        }
    }
    println!("ok");
    let mut result = deduplicate(&mat);
    sort(&mut result);

    for i in &result {
        println!("{}", i);
    }
}