use std::io;
fn main(){
    println!("Print The Guess Number");
    println!("Enter the Number");
    let mut guess=String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("you passed: {}",guess);
}