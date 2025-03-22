# Number Guessing Game in Rust

This is a simple number guessing game written in Rust, designed to demonstrate basic Rust concepts like loops, user input, error handling, and the use of external crates.

## How the Game Works

1. The game generates a random number between 1 and 100.
2. The player is prompted to guess the number.
3. After each guess, the game provides feedback:
   - "Too big" if the guess is higher than the secret number.
   - "Too low" if the guess is lower than the secret number.
   - "You won!" if the guess matches the secret number.
4. The game continues until the player guesses the number correctly.

## Prerequisites

- [Rust Programming Language](https://www.rust-lang.org/tools/install) installed.
- A Rust project with `rand` crate added as a dependency.

## Code Implementation

```rust
use rand::Rng; // `Rng` is a trait in the `rand` library
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!! Number Guessing Game !!");

    let secret = rand::thread_rng().gen_range(1..=100); // Generate random number between 1 and 100

    loop {
        println!("\n🔠 Guess a Number:");
        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        // Convert the input string to a number
        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Compare the guessed number with the secret number
        match guess_number.cmp(&secret) {
            Ordering::Equal => {
                println!("🎉 You Won!");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too low"),
        }
    }
}
```

## Key Concepts Covered

### Generating Random Numbers
- The `rand::thread_rng()` function creates a random number generator.
- `gen_range(1..=100)` generates a number between 1 and 100 (inclusive).

### Handling User Input
- `io::stdin().read_line(&mut guess)` reads input from the user and stores it in the `guess` variable.
- The `.expect()` method handles any potential errors during input reading.

### Parsing and Error Handling
- `guess.trim().parse()` converts the user input from `String` to `u32`.
- The `match` statement handles potential parsing errors and ensures that only valid numbers are processed.

### Comparing Numbers
- The `cmp()` function compares the guessed number with the secret number and returns an `Ordering` (`Less`, `Greater`, or `Equal`).

## How to Run the Game

1. Create a new Rust project:
   ```bash
   cargo new number_guessing_game
   cd number_guessing_game
   ```
2. Add the `rand` crate to your `Cargo.toml` file:
   ```toml
   [dependencies]
   rand = "0.8"
   ```
3. Replace the `main.rs` content with the code provided above.
4. Run the game:
   ```bash
   cargo run
   ```

## Additional Notes

- **Input Type in Rust:**
  Rust treats all user input as `String` by default due to strict type safety and ownership principles. This prevents buffer overflows and allows better error handling.

- **Shadowing:**
  The `guess` variable is reused after parsing, a feature known as *shadowing* in Rust.

- **External Crate Alternative:**
  In Rust, there are also external crates that handle input more efficiently:
  ```rust
  use text_io::scan;
  fn main() {
      let num: i32;
      scan!("{}", num); // Works like `cin >> num` in C++
      println!("You entered: {}", num);
  }
  ```

## Learn More
For more information on Rust programming, check out the official Rust documentation:
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)