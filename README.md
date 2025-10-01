# Compette
> [!WARNING]
> This project is a W.I.P. The functionality and features may change or be incomplete.

> [!WARNING]
> This Project is only tested on linux.

![Tests Workflow](https://github.com/vishruth-thimmaiah/compette/actions/workflows/rust_test.yml/badge.svg)


## Installation

### 1.  Install LLVM
If you use linux, dev tools for llvm can be installed from your package manager.
For example, on Arch:
```bash
pacman -S llvm
```
For more information, [click here](https://releases.llvm.org/download.html)


### 2. Install Rust
On linux:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
For more information, [click here](https://www.rust-lang.org/tools/install)

### 3. Clone the repo:
```bash
git clone https://github.com/vishruth-thimmaiah/compette.git
cd compette
```

### Build the stdlib:

```bash
cargo build --release -p stdlib
```

### Build the compiler:
```bash
cargo build --release
```
The compiler executable will be located at `target/release/compette`.

### Run tests:
```bash
cargo test
```

## Usage

### Run the compiler:
```bash
compette [COMMAND] <file> [OPTIONS]
```

### List Commands:
```bash
compette --help
```

## Syntax
Examples can be found at [```examples/```](https://github.com/vishruth-thimmaiah/compette/tree/master/examples).

### hello world:
```
import std:io

func main() i32 {
	io:println("Hello World")
	return 0
}
```
### Declaring Vars:
the 'let' keyword is used, followed by the type, variable name and value.
```
import std:io

func main() i32 {
  let i32 num = 5
  io.printint(num)
  return 0
}
```
variable are immutable unless declared otherwise.

### Mutable Vars:
A '!' after the type makes a variable mutable.
```
import std:io

func main() i32 {
  let i32! num = 5
  num = 10
  io.printint(num)
  return 0
}
```
### Math:
Basic math operations are supported.
```
import std:io

func main() i32 {
  let i32 num = 5
  let i64 num2 = 10
  io.printint(num+num2)
  return 0
}
```
In the above example, num is implicitly type casted to i64.
Implicict type casts are done between two similar data types.
For non similar type casts, (ex: f32 to i32) use explicict type casting.

### Explicit type casting:
Use '->' for explicict type casting.
```
import std:io

func main() u32 {
	let f32 a = 34.1
	let u32 b = a -> u32
	
	io:printint(b)
	
	return 0
}
```

### Conditionals:
```
func main() u32 {
  let u32 a = 2
  if a == 0 {
    return 1
  } else if a == 1 {
    return 2
  } else if a == 2 {
    return 3
  } else {
    return 0
  }
}
```
### Loops:
- An infinite Loop:
```
func main() u32 {
    let u32! a = 0
    loop {
        a = a + 1
        return a
    }
    return 0
}
```
- A conditional Loop:
```
func main() u32 {
  let u32! a = 0
  loop a < 10 {
      a = a + 1
  }
  return a
}
```
