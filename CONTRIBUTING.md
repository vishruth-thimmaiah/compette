# Contributing

## Building with Debug enabled
After cloning, run the following commands:

Building the std library:
```bash
cargo build -p stdlib
```
Building the main compiler:
```bash
cargo build
```

Running tests:
```bash
cargo test
```

Cargo run can be used to build and run in the same line:
```bash
cargo run -- run examples/fib.slpe
```
Equivalent to: 
```bash
./target/debug/sloppee run examples/fib.slpe
```

## Project Structure

- src/ -> The main Compiler code.
  - lexer/ -> handles the lexing/tokenisation of input code.
  - parser/ -> parser tokens and build an AST.
  - llvm/ -> the backend for the compiler.
    - tests/ -> test cases to cover. 

- stdlib/ -> The standard Library used by the compiler.
