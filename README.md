# rtw-cat

> [!NOTE]
> **RTW Series (Reinvent The Wheel)**
> This project is part of a series called RTW - Reinvent The Wheel. The main goal of this series is not to create alternatives to existing tools, but to deeply understand how these tools work by reimplementing them. Each project suffixed with `rtw-` is a technical and educational exploration.

## Description

`rtw-cat` is a simplified reimplementation of the Unix `cat` tool in Rust. It reads files and displays their content to standard output using efficient buffered streaming.

## What is cat?

`cat` (concatenate) is a command-line utility used to read and display file contents. It's one of the most fundamental Unix tools, often used to quickly view files or combine multiple files into one output stream.

### Classic Workflow

1. **File Opening**: The program opens the target file and obtains a file descriptor
2. **Buffered Reading**: Content is read in chunks (typically 8 KB) rather than loading the entire file
3. **Streaming Output**: Each chunk is immediately written to standard output
4. **Memory Efficiency**: Only a small buffer is kept in memory, regardless of file size

## Technical Highlights

This implementation demonstrates important systems programming concepts:

- **File descriptors**: Using `File::open()` to get a handle without loading data
- **Buffered I/O**: `BufReader` for efficient chunk-based reading (~8 KB buffer)
- **Streaming**: `io::copy()` for constant memory usage regardless of file size
- **Error handling**: Proper `Result` types and error propagation

## Usage

### Basic usage
```bash
cargo run <file_path>
```

Example:
```bash
cargo run example.txt
```

### With cargo build
```bash
cargo build --release
./target/release/rtw-cat example.txt
```

## Code Structure

- `src/main.rs`: Application entry point and error handling
- `src/lib.rs`: Core streaming logic with buffered I/O
- `src/config.rs`: Configuration and command-line argument parsing

## Tests

Run the tests:
```bash
cargo test
```

## Learning Resources

This project explores:
- How file descriptors work in operating systems
- The difference between buffered and unbuffered I/O
- Why streaming is crucial for handling large files
- Rust's ownership model applied to I/O operations
