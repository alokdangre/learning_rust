// Temperature convertor from celsius to fahrenheit and viceversa
use std::io;
// F = (C x 1.8) + 32
// C = (f - 32)/1.8
fn main() {
    loop {
        println!("Enter your query 1. for Temperature->fahrenheit, 2. fahrenheit->Temperature 3. q to quit");
        let mut query = String::new();
        io::stdin().read_line(&mut query);
        // println!("{query}");
        if query == "1\n" {
            println!("please give Temperature value");

            loop{
                let mut temp = String::new();

                io::stdin().read_line(&mut temp);
                
                let temp:f32 = match temp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {println!("Please enter a number for celsius");continue;},
                };
            let res = (temp *1.8)+32.0;
            println!("fahrenheit is {res}");break;}
        }
        else if query == "2\n" {
            
            println!("please give fahrenheit value");
            loop {
                let mut far = String::new();
                io::stdin().read_line(&mut far);
                let far:f32 = match far.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {println!("Please enter a number for fahrenheit");continue;},
                };
            let res = (far -32.0)/1.8;
            println!("Temperature is {res}");break;}
        }
        else if query == "q\n" {break;}
        else {println!("Please enter valid query");}
    }
    println!("Thanks");
}