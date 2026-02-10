# Rust Shell Implementation

[![progress-banner](https://backend.codecrafters.io/progress/shell/96e31b5b-9999-4039-ade3-42ff2fcba008)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

A POSIX-compliant shell implementation in Rust, built for the [CodeCrafters "Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

> **🙏 Special thanks to CodeCrafters** for providing the excellent learning platform and structured challenges that made building this shell implementation possible. The CodeCrafters team has created an outstanding educational experience that guides developers through building complex systems from scratch.

## Features

### Core Functionality
- **REPL Interface** - Interactive shell with command prompt and history
- **Command Parsing** - Robust parsing of shell commands with arguments
- **Process Execution** - Run external programs and manage child processes
- **Built-in Commands** - Implement common shell utilities

### Built-in Commands
- `cd <path>` - Change working directory with path resolution
- `pwd` - Print current working directory
- `echo [args...]` - Display messages with argument expansion support
- `type <command>` - Show command type information (built-in vs external)
- `history` - Display command history with optional limits and file operations
- `locate <command>` - Find executable files in PATH
- `exit` - Exit the shell

### Advanced Features
- **Command Pipelines** - Full support for command piping (`|`) with multi-stage pipelines
- **I/O Redirection** - Complete redirection support:
  - `>` - Overwrite output to file
  - `>>` - Append output to file  
  - `<` - Input from file
  - Error stream redirection (`2>`, `2>>`)
- **Auto-completion** - Tab completion for:
  - Built-in commands
  - External executables in PATH
  - File and directory paths
- **History Management** - Thread-safe history with:
  - In-memory storage
  - Navigation via arrow keys
  - History search functionality
- **Path Helper** - Utilities for path manipulation and resolution
- **External Command Execution** - Run any system command with proper process management

## Project Structure

The shell is organized into several well-structured modules:

```
src/
├── main.rs                                    # Application entry point and initialization
├── lib.rs                                     # Library crate root
├── core/                                      # Core utilities and shared functionality
│   ├── mod.rs                               # Core module definitions
│   └── utils.rs                             # Shared utility functions
├── shell/                                     # Main shell engine and REPL loop
│   ├── mod.rs                               # Shell module definitions
│   └── engine.rs                            # Shell engine with REPL implementation
├── parsing/                                   # Command parsing and tokenization
│   ├── mod.rs                               # Parsing module definitions
│   └── command_parser.rs                    # Command line parsing logic
├── commands/                                  # Command system architecture
│   ├── mod.rs                               # Command module root
│   ├── core/                                # Command infrastructure
│   │   ├── mod.rs                          # Core command definitions
│   │   ├── command_handler.rs              # Command handler trait
│   │   ├── registry.rs                     # Command registration system
│   │   ├── history_state.rs               # History state management
│   │   └── supported_command.rs            # Command enumeration and types
│   ├── handlers/                            # Individual command implementations
│   │   ├── mod.rs                          # Handler module definitions
│   │   ├── cd_command_handler.rs          # Change directory command
│   │   ├── echo_command_handler.rs        # Echo command implementation
│   │   ├── history_command_handler.rs     # History command handling
│   │   ├── locate_command_handler.rs      # Command location finder
│   │   ├── pipeline_command_handler.rs    # Pipeline command processing
│   │   ├── pwd_command_handler.rs         # Print working directory
│   │   ├── redirection_command_handler.rs # I/O redirection handling
│   │   ├── type_command_handler.rs        # Command type checker
│   │   └── unspecified_command_handler.rs # External command execution
│   └── utils/                               # Command utilities and helpers
│       ├── mod.rs                          # Utils module definitions
│       └── path_helper.rs                  # Path manipulation utilities
└── auto_complete/                             # Tab completion functionality
    ├── mod.rs                               # Auto-completion module
    └── auto_complete_helper.rs              # Tab completion implementation
```

### Key Components

- **Shell Engine** (`src/shell/engine.rs`) - Main REPL loop with rustyline integration
- **Command Registry** (`src/commands/core/registry.rs`) - Extensible command registration system  
- **Command Parser** (`src/parsing/command_parser.rs`) - Robust command line parsing
- **Handler System** (`src/commands/handlers/`) - Modular command implementations
- **History Management** (`src/commands/core/history_state.rs`) - Thread-safe command history
- **Auto-completion** (`src/auto_complete/`) - Tab completion for commands and paths

## Usage

### Running the Shell

```bash
# Run the shell
./your_program.sh

# Or using cargo
cargo run
```

### Interactive Mode

The shell provides an interactive prompt where you can type commands:

```bash
$ pwd
/home/user/project
$ ls -la
$ echo "Hello, World!"
$ cd /tmp
$ history
```

### Command Examples

```bash
# Basic commands
$ echo "Hello from Rust Shell"
$ pwd
/home/user/project

# Directory navigation
$ cd /home
$ pwd
/home

# Redirection
$ echo "test" > output.txt
$ cat < input.txt

# Pipelines
$ ls -la | grep ".rs"
$ echo "hello" | wc -c

# Auto-completion (press Tab)
$ cd /ho<Tab>  # Completes to /home/
$ echo <Tab>   # Shows available files
```

## Development

### Prerequisites
- Rust 1.92 or later
- Cargo package manager

### Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Build in release mode
cargo build --release
```

## Technical Implementation

### Dependencies
- **rustyline** - Advanced readline functionality with history and auto-completion
- **anyhow** - Error handling and result management
- **regex** - Regular expression support for parsing

### Architecture Patterns
- **Command Pattern** - Extensible command handler system
- **Registry Pattern** - Dynamic command registration and discovery
- **Thread Safety** - Arc<Mutex<>> for shared state management
- **Trait-based Design** - Modular and testable command handlers

### Performance Features
- **Zero-copy Parsing** - Efficient string handling for command parsing
- **Lazy Evaluation** - On-demand command execution and validation
- **Memory Efficient** - Minimal allocations in hot paths

## CodeCrafters Integration

This project is part of the CodeCrafters "Build Your Own Shell" challenge. The implementation follows CodeCrafters' structured learning approach and testing methodology.

### Running CodeCrafters Tests
1. Make changes to your implementation
2. Commit and push to GitHub:
   ```bash
   git commit -am "update implementation"
   git push origin master
   ```
3. Check the CodeCrafters dashboard for automated test results

### CodeCrafters Methodology
- **Incremental Development** - Features built step-by-step with validated checkpoints
- **Test-Driven Learning** - Each stage includes comprehensive test coverage
- **Best Practices** - Code quality, error handling, and performance optimization
- **Community Support** - Access to expert guidance and peer discussions

> This implementation demonstrates mastery of systems programming concepts including process management, parsing algorithms, concurrent programming, and Unix/Linux system calls - all taught through CodeCrafters' excellent curriculum.

## Contributing

Feel free to submit issues and enhancement requests!

## License

This project is open source and available under the MIT License.
