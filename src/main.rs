extern crate rand;
use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guessing game");
    
   loop {
    println!("Please input guess:");
let mut guess = String::new();
io::stdin()
.read_line(&mut guess)
.expect("Error reading line.");
let guess:u32 = guess.trim().parse();
match guess {
    Ok(num) => num,
    Err(_) => continue
}

println!("Your guess is: {} ",guess);
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big"),
    Ordering::Equal =>  {
        println!("You win!");
        break;
    }
}
   } 
}
