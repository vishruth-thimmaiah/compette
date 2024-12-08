# Sloppee
### (W.I.P)

## Installation

### Install LLVM
This can be done by installing LLVM from the official website:
https://releases.llvm.org/download.html

You can also install LLVM from your package manager.
For example, on Arch:
```bash
pacman -S llvm
```

### Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build the stdlib:

```bash
cargo build --release -p stdlib
```

### Build the compiler:
```bash
cargo build --release
```
The compiler executable will be located at `target/release/sloppee`.

## Usage

### Run the compiler:
```bash
sloppee [COMMAND] <file> [OPTIONS]
```

### List Commands:
```bash
sloppee --help
```

### Run tests:
```bash
cargo test
```
