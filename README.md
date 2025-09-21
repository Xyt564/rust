# Rust Playground 🦀

This is where I drop my test creations in Rust.  
Each folder only contains a single `main.rs` file — nothing more, nothing less.

## Getting Started

### 1. Install Rust

To build and run this project, you need [Rust](https://www.rust-lang.org/tools/install).
On most systems, you can install it using:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts, then restart your terminal or run:

```bash
source $HOME/.cargo/env
```

To confirm Rust is installed:

```bash
rustc --version
cargo --version
```

---

### 2. Build & Run with Cargo (recommended)

Cargo is Rust’s build system and package manager.

```bash
# Navigate into the project directory
cd your_project_name

(in this files case do cargo new (file name) and replace ur main.rs
with the file u downloaded from the repo)

# Build the project
cargo build

# Run the project
cargo run
```

---

### 3. Build & Run with rustc (manual option)

You can also compile the `main.rs` file directly using the Rust compiler:

```bash
rustc main.rs -o my_app


./my_app
```

---


## What to Expect
- Small experiments with Rust syntax and features
- Random ideas I wanted to try out
- Code that may or may not be polished

## Why
Just a personal sandbox for learning, testing, and having fun with Rust.

---
