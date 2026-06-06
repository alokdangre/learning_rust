// Token vault (35 min)
// A vault of tokens (Vec<String>). Claim tokens, process them, get ownership back on failure, restore failures to vault.
// Signatures — non-negotiable:
// rust
// claim — removes the token at index from vault and returns ownership of it. Out of bounds → None.
// process — if token starts with "VALID_" return Ok(token) (the processed one). Otherwise return Err(token) — giving ownership back to the caller so they can restore it.
// restore — push the token back into vault.
// audit — count how many tokens in vault currently start with "VALID_".
fn claim(vault: &mut Vec<String>, index: usize) -> Option<String> {
    if index >= vault.len() {
        return None;
    }
    Some(vault.remove(index)) 
}

fn process(token: String) -> Result<String, String> {
    if token.starts_with("VALID_") {
        Ok(token)  
    } else {
        Err(token) 
    }
}

fn restore(vault: &mut Vec<String>, token: String) {
    vault.push(token);
}

fn audit(vault: &Vec<String>) -> usize {
    let mut count = 0;
    for token in vault {
        if token.starts_with("VALID_") {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut vault: Vec<String> = vec![
        String::from("VALID_alpha"),
        String::from("junk"),
        String::from("VALID_beta"),
        String::from("trash"),
        String::from("VALID_gamma"),
    ];

    let total = vault.len();
    for _ in 0..total {
        match claim(&mut vault, 0) {
            None => println!("nothing there"),
            Some(token) => match process(token) {
                Ok(t) => println!("processed: {}", t),
                Err(t) => {
                    println!("failed:    {}", t);
                    restore(&mut vault, t);
                }
            },
        }
    }

    println!("\nFinal vault:");
    for t in &vault {
        println!("  {}", t);
    }
    println!("Valid tokens remaining: {}", audit(&vault));
}