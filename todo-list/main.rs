use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use serde::{Serialize, Deserialize}; // ✅ required by serde

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    status: Status,
}

const FILE_PATH: &str = "tasks.json"; // 🔒 JSON file name

fn main() {
    // 🔄 Load existing tasks from file if it exists
    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n--- TODO APP ---");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Done");
        println!("4. Delete Task");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter task description:");
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();

                let task = Task {
                    description: desc.trim().to_string(),
                    status: Status::Pending,
                };
                tasks.push(task);
                save_tasks(&tasks); // 💾 Save to file
                println!("Task added!");
            }
            "2" => {
                println!("\nYour Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    let status = match task.status {
                        Status::Pending => "Pending",
                        Status::Done => "Done",
                    };
                    println!("{}. [{}] {}", i + 1, status, task.description);
                }
            }
            "3" => {
                println!("Enter task number to mark as done:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].status = Status::Done;
                        save_tasks(&tasks); // 💾 Save change
                        println!("Task marked as done!");
                    } else {
                        println!("Invalid task number.");
                    }
                }
            }
            "4" => {
                println!("Enter task number to delete:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks.remove(index - 1);
                        save_tasks(&tasks); // 💾 Save change
                        println!("Task deleted!");
                    } else {
                        println!("Invalid task number.");
                    }
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice.");
            }
        }
    }
}

// 📤 Save tasks to file
fn save_tasks(tasks: &Vec<Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // overwrite
        .open(FILE_PATH)
        .expect("Unable to open file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &tasks).expect("Failed to write JSON");
}

// 📥 Load tasks from file (if exists)
fn load_tasks() -> Vec<Task> {
    let file = File::open(FILE_PATH);
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        let tasks: Result<Vec<Task>, _> = serde_json::from_reader(reader);
        match tasks {
            Ok(t) => t,
            Err(_) => {
                println!("⚠️ Warning: Corrupted JSON. Starting fresh.");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    }
}
