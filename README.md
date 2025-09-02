# RS-Todo

A simple command-line todo application written in Rust. Manage your tasks efficiently from the terminal with persistent storage.

## Features

- ✅ Add new tasks
- 📋 List all tasks
- ❌ Remove tasks by ID
- 💾 Persistent storage using JSON
- 🚀 Fast and lightweight

## Prerequisites

Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
# Verify Rust installation
rustc --version
cargo --version
```

## Installation

### Method 1 : Clone and Build from Source

1. **Clone the repository:**
   ```bash
   git clone <your-repository-url>
   cd todo-cli
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the application:**
   ```bash
   # Run directly with cargo
   cargo run -- <command>
   
   # Or use the compiled binary
   ./target/release/todo-cli <command>

   # copy the binary into the PATH
   # Best method on LINUX/MacOS
   sudo cp target/release/todo-cli /usr/local/bin/todo-cli
   ```
### Method 2 : Install from crates.io

   ```bash
   cargo install rstodo
   ```

## Usage

### Add a Task
```bash
todo-cli add "Buy groceries"
todo-cli add "Complete Rust project"
todo-cli add "Call mom"
```

### List All Tasks
```bash
todo-cli list
```

Output example:
```
0: Buy groceries
1: Complete Rust project
2: Call mom
```

### Remove a Task
```bash
# Remove task with ID 1
todo-cli remove 1
```

### Help
```bash
todo-cli --help
```

## Examples

```bash
# Add some tasks
$ todo-cli add "Learn Rust"
Task added.

$ todo-cli add "Build a CLI app"
Task added.

$ todo-cli add "Deploy to production"
Task added.

# List all tasks
$ todo-cli list
0: Learn Rust
1: Build a CLI app
2: Deploy to production

# Remove completed task
$ todo-cli remove 0
Task removed.

# Check remaining tasks
$ todo-cli list
0: Build a CLI app
1: Deploy to production
```

## Data Storage

Tasks are automatically saved to `todos.json` in the current working directory. The file is created automatically when you add your first task.

Example `todos.json` structure:
```json
{
  "todos": [
    "Build a CLI app",
    "Deploy to production"
  ]
}
```

## Development

### Project Structure
```
todo-cli/
├── src/
│   └── main.rs          # Main application logic
├── Cargo.toml           # Project dependencies and metadata
├── Cargo.lock           # Dependency lock file
├── todos.json           # Task storage (created at runtime)
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

### Dependencies
- **clap** - Command line argument parsing
- **serde** - Serialization/deserialization
- **serde_json** - JSON support


### Building for Release
```bash
cargo build --release
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request


## Future Enhancements

- [ ] Task priorities
- [ ] Due dates
- [ ] Task categories/tags
- [ ] Mark tasks as complete without removing
- [ ] Export to different formats
- [ ] Configuration file support
- [ ] Colored output