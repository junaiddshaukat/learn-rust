use std::io;
use rand;

fn main() {
    println!("!! Number Guessing Game !!");
    println!("🔠 Guess a Number : ");

    let mut guess = String::new(); // mutable string :: means new is the function of the string

    io::stdin().read_line(&mut guess).expect("Failed to read the input"); // we use & -> reference and mut -> mutable reference because by default the reference is Immutable
    // .except() -> means if the line fail then it print the msg and it crash the program

    print!("you guessed : {}", guess);


}
