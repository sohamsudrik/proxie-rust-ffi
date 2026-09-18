# Proxie Rust FFI

A Rust wrapper around a C++ string utility library using
Rust FFI and bindgen.

## Features

The project provides the following string utilities:

- String length
- String reversal
- Vowel counting
- Uppercase conversion

## Project Architecture

```text
Rust Safe API
      |
      v
Rust FFI Layer
      |
      v
bindgen Generated Bindings
      |
      v
C-Compatible Header
      |
      v
C++ Implementation
```

## Running the Project

### Rust FFI Application
Run the Rust application demonstrating the C++ FFI integration:
```bash
cargo run
```

### Run Tests
Execute the integration test suite:
```bash
cargo test
```

### Standalone C++ Demo
Compile and run the standalone C++ program:
```bash
g++ -std=c++11 CPP/main.cpp CPP/lib.cpp -o CPP/cpp_demo.exe
./CPP/cpp_demo.exe
```