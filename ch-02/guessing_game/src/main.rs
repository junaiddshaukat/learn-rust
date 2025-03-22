use rand::Rng; // ::Rng is a trait in the rand library
use std::cmp::Ordering;
use std::io; //for comparing
fn main() {
    println!("!! Number Guessing Game !!");

    let secret = rand::rng().random_range(1..=100);

    loop {
        println!("🔠 Guess a Number ");
        let mut guess = String::new();
        // mutable string :: means new is the function of the string
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        // we use & -> reference and mut -> mutable reference because by default the reference is Immutable
        // .except() -> means if the line fail then it print the msg and it crash the program

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid input");
                continue;
            }
        };

        // here we can use the above same guess variable which is called shoadowing in rust and in rest of the program this is used

        // let guess_number: u32 = guess.parse().expect("Error in parsing");
        // when we enter a number in input like 32 it will be 32\n because of enter so that this line gives error so we use the trim function and this gives error because we are paring before nput

        match guess_number.cmp(&secret) {
            Ordering::Equal => {
                print!("You Won!");
                break;
            }
            Ordering::Greater => print!("Too big\n"),
            Ordering::Less => print!("too low\n"),
        }
    }
}

// By default rust takes all input in string because Rust follows strict type safety and ownership principles. The read_line() function reads input as a String to prevent buffer overflows and allows better error handling.
// we can use external crate to read number input like c++ but it is not standard way to do
/*
use text_io::scan;

fn main() {
    let num: i32;
    scan!("{}", num); // Works like `cin >> num` in C++
    println!("You entered: {}", num);
    }
    */

/*
    guess.trim() → Removes any leading/trailing whitespace (like \n,\r).

.parse::<u32>() → Converts the string into a u32 (unsigned 32-bit integer).

.expect("Error in parsing") → Crashes the program if parsing fails.
     */
