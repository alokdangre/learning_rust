fn find(list: &Vec<String>, item: &str) -> Option<usize> {
    for i in 0..list.len() {
        if list[i] == item {
            return Some(i);
        }
    }
    None
}

fn reserve(stock: &mut Vec<String>, reserved: &mut Vec<String>, item: &str) -> bool {
    match find(stock, item) {
        None => false,
        Some(i) => {
            reserved.push(stock.remove(i));
            true
        }
    }
}

fn release(reserved: &mut Vec<String>, stock: &mut Vec<String>, item: &str) -> bool {
    match find(reserved, item) {
        None => false,
        Some(i) => {
            stock.push(reserved.remove(i));
            true
        }
    }
}

fn damage(reserved: &mut Vec<String>, damaged: &mut Vec<String>, item: &str) -> bool {
    match find(reserved, item) {
        None => false,
        Some(i) => {
            damaged.push(reserved.remove(i));
            true
        }
    }
}

fn restock(damaged: &mut Vec<String>, stock: &mut Vec<String>) {
    while let Some(item) = damaged.pop() {
        stock.push(item);
    }
}

fn report(stock: &Vec<String>, reserved: &Vec<String>, damaged: &Vec<String>) {
    println!("stock    ({}): {:?}", stock.len(), stock);
    println!("reserved ({}): {:?}", reserved.len(), reserved);
    println!("damaged  ({}): {:?}", damaged.len(), damaged);
}

fn main() {
    let mut stock: Vec<String> = vec![
        String::from("hammer"),
        String::from("drill"),
        String::from("saw"),
        String::from("wrench"),
        String::from("hammer"),
    ];
    let mut reserved: Vec<String> = vec![];
    let mut damaged: Vec<String> = vec![];

    println!("reserve 'drill':   {}", reserve(&mut stock, &mut reserved, "drill"));
    println!("reserve 'saw':     {}", reserve(&mut stock, &mut reserved, "saw"));
    println!("reserve 'missing': {}", reserve(&mut stock, &mut reserved, "missing"));
    println!("damage  'saw':     {}", damage(&mut reserved, &mut damaged, "saw"));
    println!("release 'drill':   {}", release(&mut reserved, &mut stock, "drill"));
    restock(&mut damaged, &mut stock);

    println!();
    report(&stock, &reserved, &damaged);
}