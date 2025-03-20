use std::io;

fn main() {
    println!("!! Number Guessing Game !!");
    println!("🔠 Guess a Number : ");

    let mut guess = String::new(); // mutable string :: means new is the function of the string

    let a = io::stdin().read_line(&mut guess); // we use & -> reference and mut -> mutable reference because by default the reference is Immutable


}
