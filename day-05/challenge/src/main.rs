// Stack calculator (15 min)
// Read from stdin in a loop:

// A number → push it onto a Vec<i32> stack
// "pop" → remove and print the top value (print "empty" if stack is empty)
// "sum" → print the sum of everything on the stack
// "quit" → exit

use std::io;

fn main() {
    let mut mat :Vec<i32> = vec![];
    println!("you have push, pop, sum, quit queries, type any of these");
    loop {
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Failed");
        let size = mat.len();
        if query=="pop\n" {
            if size==0 {
                println!("empty");
            }
            else {
                mat.pop();
                let n = mat.len();
                if n!=0 {println!("{} ", mat[n-1]);}
                else {println!("empty");}
            }
        }
        else if query=="push\n" {
            let mut num = String::new();
            io::stdin().read_line(&mut num);
            let mut num:i32 = match num.trim().parse() {
                Ok(n) => n,
                Err(_) => continue,
            };
            mat.push(num);
        }
        else if query=="sum\n" {
            let mut sum:i32 = 0;
            for i in 0..size {
                sum += mat[i];
            }
            println!("sum is {}", sum);
        }
        else if query=="quit\n" {
            break;
        }
        else {
            println!("invalid query");
        }
    }
}