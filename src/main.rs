use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

const FILE: &str = "todos.json";

#[derive(Serialize, Deserialize)]
struct TodoList {
    todos: Vec<String>,
}

#[derive(Parser)]
#[command(name = "clap-demo")]
#[command(about = "Bare bones Clap example", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Remove { id: usize },
}

fn load_todos() -> Vec<String> {
    if Path::new(FILE).exists() {
        let data = fs::read_to_string(FILE).unwrap_or_default();
        serde_json::from_str::<TodoList>(&data)
            .map(|list| list.todos)
            .unwrap_or_default()
    } else {
        vec![]
    }
}

fn save_todos(todos: &[String]) -> io::Result<()> {
    let list = TodoList {
        todos: todos.to_vec(),
    };
    let json = serde_json::to_string_pretty(&list).unwrap();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE)?;
    file.write_all(json.as_bytes())
}

fn main() {
    let cli = Cli::parse();
    let mut todos = load_todos();

    match cli.command {
        Commands::Add { task } => {
            todos.push(task);
            if let Err(e) = save_todos(&todos) {
                eprintln!("Failed to save todos: {}", e);
            } else {
                println!("Task added.");
            }
        }
        Commands::List => {
            if todos.is_empty() {
                println!("No tasks found.");
            } else {
                for (i, task) in todos.iter().enumerate() {
                    println!("{}: {}", i, task);
                }
            }
        }
        Commands::Remove { id } => {
            if id < todos.len() {
                todos.remove(id);
                if let Err(e) = save_todos(&todos) {
                    eprintln!("Failed to save todos: {}", e);
                } else {
                    println!("Task removed.");
                }
            } else {
                println!("Invalid task ID.");
            }
        }
    }
}
