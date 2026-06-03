//  Score tracker (30-minute timer, blank cargo new, docs allowed)
// You have a Vec<i32> of player scores. Write four functions, and the hard rule is: no moving and no .clone() — everything happens through borrows.

// highest(scores: &Vec<i32>) -> Option<i32> — the max, without taking ownership.
// total(scores: &Vec<i32>) -> i32 — the sum, borrowing.
// add_bonus(scores: &mut Vec<i32>, bonus: i32) — add bonus to every score, in place.
// In main: build the vec, print highest and total, call add_bonus, then print highest and total again — proving the vec is still usable after you borrowed it.

use std::io;

fn highest(scores: &Vec<i32>) -> i32 {
    let mut maxi:i32 = -1000;
    for s in scores {
        if maxi < *s {
            maxi = *s;
        }
    }
    maxi
}

fn total(scores: &Vec<i32>) -> i32 {
    let mut sum:i32 = 0;
    for s in scores {
        sum += *s;
    }
    sum
}

fn add_bonus(scores: &mut Vec<i32>, bonus: i32) {
    for s in scores {
        *s += bonus;
    }
}

fn main() {
    let mut score : Vec<i32> = vec![];
    println!("Add scores");
    loop{
        let mut inpu = String::new();
        io::stdin().read_line(&mut inpu);
        let inpu:i32 = match inpu.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if inpu=="done\n" {break;}
                else {continue;}
            }
        };
        println!("hello {}",inpu);
        score.push(inpu);
    }
    for s in score {
        println!("{} ",s);
    }
    let high = highest(&score);
    let tot = total(&score);
    add_bonus(&mut score,10);
    println!("highest score is {}",high);
    println!("total score is {}",tot);
    println!("scores in vector after adding bonus to all is :");
    for s in score {
        println!("{} ",s);
    }
}