extern crate rand;
use std::io;
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guessing game");
    
    println!("Please input guess:");
let mut guess = String::new();
io::stdin()
.read_line(&mut guess)
.expect("Error reading line.");

println!("Your guess is: {} ",guess);
}
