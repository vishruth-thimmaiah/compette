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

## Usage

### Build a binary:
```bash
cargo build
```

### Run the compiler:
```bash
sloppee [COMMAND] <file> [OPTIONS]
```

### Run tests:
```bash
cargo test
```
