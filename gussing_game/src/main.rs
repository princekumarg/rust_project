use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Print The Guess Number");
    let secret_number = rand::thread_rng().gen_range(0..=100);
    loop {
        println!("Enter the Guess Number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(err)=>{
                println!("please Enter a Valid Input");
                continue;
            }
        }; //shadowing
        println!("you passed: {guess}");
        println!("your secret number is hidden");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less Number"),
            Ordering::Greater => println!("Greater Number"),
            Ordering::Equal => {
                println!("You Guess The Correct Number");
                break;
            }
        }
    }
}
