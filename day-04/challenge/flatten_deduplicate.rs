// Flatten and deduplicate
// Input:  vec![vec![3,1,2], vec![2,4,1], vec![5]]
// Output: vec![1, 2, 3, 4, 5]  // unique, sorted
// Rules: function takes &[Vec<i32>] — borrow only, no cloning the outer structure. Return a new Vec<i32>.

use std::io;
fn deduplicate(m: &[Vec<i32>]) -> Vec<i32> {
    let mut flat: Vec<i32> = vec![];
    for inner in m {
        for &k in inner {
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
    let mut mat : Vec<Vec<i32>> = vec![];
    loop {
        let mut inner_vec : Vec<i32> = vec![];
        let mut b = false;
        loop {
            let mut inpu = String::new();
            io::stdin().read_line(&mut inpu);
            let inpu:i32 = match inpu.trim().parse() {
                Ok(num) => num,
                Err(_) => {if inpu=="done\n" {b = true;break;}
                else if inpu=="d" {break;}
                else {continue;}},
            };
            inner_vec.push(inpu);
        }
        if b {break;}
        mat.push(inner_vec);
    }
    let mut result = deduplicate(&mat);
    sort(&mut result);
    for i in &result {
        println!("{} ",i);
    }
}