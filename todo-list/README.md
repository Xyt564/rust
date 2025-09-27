# Todo List

A simple command-line **To-Do List** application written in Rust.

## Overview

This project is a basic Rust program that allows you to manage a to-do list through the command line. It demonstrates Rust concepts such as:

* Reading and writing files
* Working with JSON serialization/deserialization
* Handling user input
* Structuring a CLI application

The to-do list tasks are saved in a JSON file, so your tasks persist between runs.

## Features

* Add new tasks
* View all current tasks with their status (completed or not)
* Mark tasks as completed
* Delete tasks from the list
* Persistent storage using a JSON file

## Dependencies

This project relies on the following crates for serialization and JSON handling:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Make sure these are included in your `Cargo.toml` file before running the project.

## Getting Started

### Prerequisites

You need Rust installed. If you don't have it installed, use the following commands:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify the installation:

```bash
rustc --version
cargo --version
```

### Running the Todo List

1. Clone the repository and navigate to the todo-list folder:

```bash
git clone https://github.com/Xyt564/rust.git
cd rust/todo-list
```

2. Ensure the dependencies are present in `Cargo.toml`.

3. Run the program using Cargo:

```bash
cargo run
```

## Usage

When running, the program will present a menu with options like:

* **Add task**: Enter a new to-do item.
* **View tasks**: See all current tasks with their completion status.
* **Complete task**: Mark a specific task as done.
* **Delete task**: Remove a task from the list.
* **Exit**: Quit the program safely.

Tasks are stored in a JSON file (usually in the project directory), allowing your list to persist between sessions.

## Example Session

```plaintext
Welcome to the Todo List!

Select an option:
1. Add Task
2. View Tasks
3. Complete Task
4. Delete Task
5. Exit

Enter choice: 1
Enter task description: Buy groceries

Task added!

Select an option:
1. Add Task
2. View Tasks
3. Complete Task
4. Delete Task
5. Exit

Enter choice: 2

Your Tasks:
1. [ ] Buy groceries

Select an option:
...
```
