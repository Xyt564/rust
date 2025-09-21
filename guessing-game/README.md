# Rust Guessing Game 🎲

A simple Rust command-line program that asks the user to input a guess and then prints it back.  
This is a starting point for learning Rust basics such as input/output, variables, and string handling.

## How it Works
1. The program prompts the user to enter a guess.
2. It reads the input from the terminal.
3. It prints back the value the user entered.

Example run:
```

Guess the number!
Please input your guess.
42
You guessed: 42

````

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Run the Program
Clone the repository and run: cargo new (your folder name)

(replace ur main.rs with the one in the repo)

cargo build

```bash
cargo run
````

### Build the Program

To compile the binary:

```bash
cargo build --release
```

The executable will be located in `target/release/`.

## Next Steps

This is just the foundation for a guessing game.
Future improvements could include:

* Generating a random secret number.
* Checking if the guess matches the secret number.
* Giving hints like "Too small!" or "Too big!".

---

Made with ❤️ while learning Rust.

```
