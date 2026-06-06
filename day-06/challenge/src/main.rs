// Contact book — parallel Vec REPL (40 min)
// Two parallel Vec<String>: names and phones. Same index = same person. Implement these five functions — signatures are non-negotiable, that's the whole point:
// rustfn add(names: &mut Vec<String>, phones: &mut Vec<String>, name: String, phone: String)
// REPL — one command per line, args on the next lines:
// add
// Alice
// 555-1234
// find
// Alice
// update
// Alice
// 555-9999
// remove
// Bob
// list
// quit

use std::io;

fn find(names: &Vec<String>, phones: &Vec<String>, query: &str) -> i32 {
    let n = names.len();
    for i in 0..n {
        if names[i]==query {
            println!("phone no. is {} ", phones[i]);
            return i.try_into().unwrap();
        }
    }
    println!("not found");
    -1
}
fn remove(names: &mut Vec<String>, phones: &mut Vec<String>, target: i32)  {
    names.remove(target.try_into().unwrap());
    phones.remove(target.try_into().unwrap());
}
fn update(names: &Vec<String>, phones: &mut Vec<String>, target: i32, new_phone: String)  {
    let n = phones.len();
    for i in 0..n {
        if i==target.try_into().unwrap() {
            phones[i]=new_phone.clone();
        }
    }
}
fn list(names: &Vec<String>, phones: &Vec<String>) {
    let n = names.len();
    for i in 0..n {
        println!("{}'s phone no. is {}",names[i],phones[i]);
    }
}

fn main() {
    let mut names:Vec<String> = vec![];
    let mut phones:Vec<String> = vec![];
    loop {
        let mut query = String::new();
        io::stdin().read_line(&mut query);
        if query=="find\n" {
                let mut q = String::new();
                io::stdin().read_line(&mut q);
                find(&names,&phones, &q);
            }
        else if query=="remove\n" {
                let mut q = String::new();
                io::stdin().read_line(&mut q);
                let index:i32 = find(&names,&phones,&q);
                remove(&mut names, &mut phones, index);
            }
        else if query=="update\n" {
                let mut q = String::new();
                io::stdin().read_line(&mut q);
                let mut new_phone = String::new();
                io::stdin().read_line(&mut new_phone);
                let index:i32 = find(&names,&phones,&q);
                update(&names,&mut phones,index,new_phone);
            }
        else if query=="list\n" {
                list(&names,&phones);
            }
        else if query=="quit\n" {
                break
            }
        else if query=="add\n" {
            println!("enter name");
            let mut q = String::new();
                io::stdin().read_line(&mut q);
                println!("enter phone no.");
                let mut new_phone = String::new();
                io::stdin().read_line(&mut new_phone);
                names.push(q);
                phones.push(new_phone);
        }
    }
}