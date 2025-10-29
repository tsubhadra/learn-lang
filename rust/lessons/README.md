# Rust Lessons - Hello World Example

This directory contains a Rust Hello World example demonstrating basic Rust program structure.

## Files

### Source Files
- **`01-hello-world.rs`** - Rust source code that prints "Hello world!"

### Compiled Files (Generated)
- **`01-hello-world`** - Compiled executable binary (~400KB)

## Prerequisites

### Rust Installation

**Check if Rust is installed:**
```bash
rustc --version
cargo --version
rustup --version
```

**If not installed:**

**macOS/Linux (recommended method):**
```bash
# Install using rustup (official installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the on-screen instructions
# Restart terminal or run:
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

**macOS (using Homebrew - alternative):**
```bash
brew install rust
```

**Windows:**
```powershell
# Download and run rustup-init.exe from:
# https://rustup.rs/

# Or using Chocolatey:
choco install rust
```

**Linux (alternative package managers):**
```bash
# Arch Linux
sudo pacman -S rust

# Fedora
sudo dnf install rust cargo

# Debian/Ubuntu (may be outdated, rustup preferred)
sudo apt install rustc cargo
```

## How to Compile and Run Rust Programs

### Method 1: Using rustc (Direct Compilation)

```bash
# Navigate to directory
cd /Users/suthirumavalavan/learn/learn-lang/rust/lessons

# Compile the source code
rustc 01-hello-world.rs

# Run the executable
./01-hello-world
```

**Output:**
```
Hello world!
```

### Method 2: With Optimization (Release Mode)

```bash
# Compile with optimizations
rustc -O 01-hello-world.rs

# Or with specific optimization level
rustc -C opt-level=3 01-hello-world.rs

# Run
./01-hello-world
```

### Method 3: Compile and Run in One Command

```bash
# Compile and run immediately
rustc 01-hello-world.rs && ./01-hello-world
```

### Method 4: Using Cargo (Project Management - Recommended for Real Projects)

```bash
# Create a new Cargo project
cargo new hello-world
cd hello-world

# Replace src/main.rs with your code

# Build the project
cargo build

# Run the project
cargo run

# Build with optimizations (release mode)
cargo build --release
cargo run --release
```

### Method 5: Check Syntax Without Compiling

```bash
# Check for errors without producing executable
cargo check

# Or with rustc
rustc --parse-only 01-hello-world.rs
```

## Understanding Rust Files

### File Extensions

```
01-hello-world.rs          # Source code (human-readable)
01-hello-world             # Compiled binary (executable)
```

### Rust vs Other Languages

**C++ (requires explicit includes and namespace):**
```cpp
#include <iostream>

int main() {
    std::cout << "Hello world!" << std::endl;
    return 0;
}
```
```bash
g++ hello.cpp -o hello
./hello
```

**Java (requires class and method boilerplate):**
```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}
```
```bash
javac HelloWorld.java
java HelloWorld
```

**Rust (simple and safe):**
```rust
fn main() {
    println!("Hello world!");
}
```
```bash
rustc hello.rs
./hello
```

**Key Differences:**
- ✅ No classes required (like C, unlike Java)
- ✅ No manual memory management (unlike C++)
- ✅ No garbage collector (unlike Java, Python)
- ✅ Memory safety guaranteed at compile time
- ✅ Zero-cost abstractions
- ✅ Fearless concurrency

## Program Structure Explained

### Minimal Rust Program

```rust
fn main() {
    println!("Hello world!");
}
```

That's it! The simplest Rust program.

### Anatomy of a Rust Program

```rust
// Comments start with //
/* Multi-line comments
   use /* and */ */

/// Documentation comments (generates docs)
/// Use three slashes for item documentation

// Function definition
fn main() {              // fn = function keyword
    // println! is a macro (note the !)
    // Macros are expanded at compile time
    println!("Hello world!");  // Semicolon required
}  // No semicolon after }
```

### Key Components

1. **`fn` keyword** - Defines functions
2. **`main()`** - Entry point (required for executables)
3. **`println!()`** - Macro for printing (note the `!`)
4. **Semicolons** - Required at end of statements
5. **Comments** - `//` for single line, `/* */` for multi-line

### println! vs print!

```rust
fn main() {
    // println! adds a newline
    println!("Hello");
    println!("world");
    // Output:
    // Hello
    // world
    
    // print! does not add newline
    print!("Hello ");
    print!("world");
    // Output: Hello world
}
```

## Rust Compilation Process

### Compilation Flow

```
Source Code (.rs)
        ↓
    [rustc]  ← Rust compiler
        ↓
 LLVM IR  ← Intermediate representation
        ↓
    [LLVM]  ← Optimization and code generation
        ↓
 Native Binary  ← Executable for your platform
        ↓
   Execute  ← Run directly on CPU
        ↓
 "Hello world!"  ← Output
```

**Compilation characteristics:**
- Statically compiled (no runtime needed)
- Platform-specific binary
- Optimized by LLVM
- Type checking at compile time
- Borrow checker ensures memory safety
- No garbage collection overhead

### Build Artifacts

```bash
$ rustc 01-hello-world.rs
$ ls -lh
-rwxr-xr-x  431K  01-hello-world      # Executable
-rw-r--r--  807B  01-hello-world.rs   # Source
```

**Why is the binary so large for a simple program?**
- Includes standard library components
- Debug symbols (in debug builds)
- Static linking by default
- Use `--release` for smaller, optimized builds

## Different Ways to Print in Rust

### Basic Printing

```rust
fn main() {
    // Simple print with newline
    println!("Hello world!");
    
    // Print without newline
    print!("Hello ");
    print!("world!\n");
    
    // Print to stderr
    eprintln!("Error message");
}
```

### String Formatting

```rust
fn main() {
    // 1. Positional parameters
    println!("{} {}!", "Hello", "world");
    
    // 2. Named parameters
    println!("{greeting} {name}!", greeting = "Hello", name = "world");
    
    // 3. Variables
    let message = "world";
    println!("Hello {}!", message);
    
    // 4. Multiple uses
    println!("{0} {1}, {1} {0}!", "Hello", "world");
    // Output: Hello world, world Hello!
    
    // 5. Debug formatting {:?}
    let numbers = vec![1, 2, 3];
    println!("{:?}", numbers);  // [1, 2, 3]
    
    // 6. Pretty debug formatting {:#?}
    println!("{:#?}", numbers);
    
    // 7. Display vs Debug
    // {} uses Display trait
    // {:?} uses Debug trait
}
```

### Advanced Formatting

```rust
fn main() {
    // Width specification
    println!("{:5}", "Hi");      // "Hi   "
    
    // Padding with zeros
    println!("{:05}", 42);       // "00042"
    
    // Hexadecimal
    println!("{:x}", 255);       // "ff"
    println!("{:X}", 255);       // "FF"
    
    // Binary
    println!("{:b}", 5);         // "101"
    
    // Alignment
    println!("{:<10}", "left");  // "left      "
    println!("{:>10}", "right"); // "     right"
    println!("{:^10}", "center");// "  center  "
}
```

## Rust Features Demonstrated

### 1. Type Safety

```rust
fn main() {
    // Rust infers types
    let x = 5;           // x is i32 (32-bit integer)
    let y = 5.0;         // y is f64 (64-bit float)
    
    // Explicit types
    let z: u32 = 10;     // u32 = unsigned 32-bit integer
    
    // Type errors caught at compile time
    // let result = x + "hello";  // ERROR: cannot add integer and string
}
```

### 2. Immutability by Default

```rust
fn main() {
    // Immutable by default
    let x = 5;
    // x = 6;  // ERROR: cannot assign twice to immutable variable
    
    // Mutable when needed
    let mut y = 5;
    y = 6;  // OK
    println!("{}", y);
}
```

### 3. Ownership (Rust's Superpower)

```rust
fn main() {
    // String ownership
    let s1 = String::from("hello");
    let s2 = s1;  // s1 is moved to s2
    // println!("{}", s1);  // ERROR: s1 no longer valid
    println!("{}", s2);     // OK
    
    // Clone if you need a copy
    let s3 = s2.clone();
    println!("{} {}", s2, s3);  // Both valid
}
```

### 4. Macros vs Functions

```rust
fn main() {
    // println! is a macro (note the !)
    println!("Hello");  // Macro
    
    // Functions don't have !
    // std::io::stdout().write_all(b"Hello\n");  // Function
}
```

**Why macros?**
- Variable number of arguments
- Compile-time code generation
- Type checking at expansion time
- More powerful than functions

## rustc vs cargo

### rustc (Rust Compiler)

**Use for:**
- Single-file programs
- Quick tests
- Learning Rust basics
- Simple scripts

**Limitations:**
- No dependency management
- No project structure
- Manual compilation
- No build scripts

**Example:**
```bash
rustc hello.rs
./hello
```

### cargo (Rust Package Manager)

**Use for:**
- Multi-file projects
- External dependencies
- Testing and benchmarking
- Publishing crates
- Real-world applications

**Features:**
- Dependency management
- Build system
- Testing framework
- Documentation generator
- Package registry access

**Example:**
```bash
cargo new my_project
cd my_project
cargo run
```

## Common Errors and Solutions

### 1. "rustc: command not found"

**Solution:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Restart terminal or run:
source $HOME/.cargo/env

# Verify
rustc --version
```

### 2. "main function not found"

**Problem:** Missing or misspelled `main` function

**Solution:**
```rust
// Wrong
fun main() {  // 'fun' instead of 'fn'
    println!("Hello");
}

// Correct
fn main() {
    println!("Hello");
}
```

### 3. "expected `;`"

**Problem:** Missing semicolon

**Solution:**
```rust
// Wrong
fn main() {
    println!("Hello")  // Missing semicolon
}

// Correct
fn main() {
    println!("Hello");  // Semicolon added
}
```

### 4. "cannot find macro `println` in this scope"

**Problem:** Typo in macro name

**Solution:**
```rust
// Wrong
fn main() {
    printline!("Hello");  // Wrong name
}

// Correct
fn main() {
    println!("Hello");    // Correct name
}
```

### 5. "expected `{`, found `(`"

**Problem:** Missing curly braces

**Solution:**
```rust
// Wrong
fn main()
    println!("Hello");

// Correct
fn main() {
    println!("Hello");
}
```

### 6. Compilation is slow

**Solution:**
```bash
# Use cargo for incremental compilation
cargo build

# Or install sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache
```

### 7. Binary is too large

**Solution:**
```bash
# Build in release mode
rustc -O hello.rs

# Or with cargo
cargo build --release

# Strip debug symbols
strip hello

# Check size
ls -lh hello
```

## File Sizes Comparison

```bash
$ ls -lh
-rwxr-xr-x  431K  01-hello-world           # Debug build
-rwxr-xr-x  287K  01-hello-world.release   # Release build (-O)
-rwxr-xr-x  253K  01-hello-world.stripped  # Stripped binary
-rw-r--r--  807B  01-hello-world.rs        # Source code
```

## Viewing Compiled Output (Advanced)

### Assembly Code

```bash
# Generate assembly
rustc --emit asm 01-hello-world.rs

# View assembly
less 01-hello-world.s
```

### LLVM IR

```bash
# Generate LLVM intermediate representation
rustc --emit llvm-ir 01-hello-world.rs

# View IR
less 01-hello-world.ll
```

### Disassemble Binary

```bash
# View machine code
objdump -d 01-hello-world | less

# Or use rust-specific tool
cargo install cargo-show-asm
cargo asm my_function
```

## Rust Editions

Rust releases new "editions" with breaking changes:

- **Rust 2015** - Original edition
- **Rust 2018** - Modern edition (recommended)
- **Rust 2021** - Latest edition

**Specify edition:**
```bash
# In Cargo.toml
[package]
edition = "2021"

# With rustc
rustc --edition 2021 hello.rs
```

## Next Steps

After mastering Hello World, explore:

1. **Variables and Mutability**
   - let bindings
   - mut keyword
   - Constants

2. **Data Types**
   - Scalar types (integers, floats, bool, char)
   - Compound types (tuples, arrays)
   - String vs &str

3. **Control Flow**
   - if/else expressions
   - loop, while, for
   - Pattern matching

4. **Functions**
   - Parameters and return values
   - Expressions vs statements
   - Closures

5. **Ownership and Borrowing**
   - Move semantics
   - References
   - Lifetimes

6. **Structs and Enums**
   - Defining custom types
   - Methods and associated functions
   - Pattern matching with enums

7. **Error Handling**
   - Result<T, E>
   - Option<T>
   - The ? operator

8. **Collections**
   - Vec<T>, HashMap<K, V>, String
   - Iterators

9. **Modules and Crates**
   - Organizing code
   - Using external crates
   - Publishing to crates.io

10. **Advanced Features**
    - Traits and generics
    - Smart pointers
    - Concurrency
    - Macros

## Resources

- **The Rust Programming Language (The Book)**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings (Interactive Exercises)**: https://github.com/rust-lang/rustlings
- **Rust Playground**: https://play.rust-lang.org/
- **Rust Standard Library Docs**: https://doc.rust-lang.org/std/
- **This Week in Rust**: https://this-week-in-rust.org/
- **Project Setup Guide**: See `../docs/initial-setup.txt`

## Tips

1. **Use rustup**: Official toolchain manager
2. **Learn ownership**: Rust's key feature
3. **Read compiler errors**: They're very helpful
4. **Use cargo**: Even for small projects
5. **Install rust-analyzer**: Best IDE support
6. **Use clippy**: Rust linter for best practices
7. **Format with rustfmt**: Consistent code style
8. **Read The Book**: Official Rust documentation
9. **Practice with Rustlings**: Interactive exercises
10. **Join the community**: Rust Discord, forums, Reddit

## Rust Advantages

- 🚀 **Performance**: As fast as C/C++
- 🔒 **Safety**: Memory safe without garbage collection
- ⚡ **Concurrency**: Fearless concurrency
- 🎯 **Zero-cost abstractions**: High-level features with no runtime overhead
- 📦 **Great tooling**: cargo, rustfmt, clippy, rust-analyzer
- 🌍 **Cross-platform**: Compile to many targets
- 📚 **Excellent documentation**: The Book, docs, examples
- 👥 **Friendly community**: Helpful and welcoming

## Summary

- ✅ Rust compiles to native code (no VM/interpreter)
- ✅ `rustc` is the compiler, `cargo` is the package manager
- ✅ `fn main()` is the entry point
- ✅ `println!` is a macro (note the `!`)
- ✅ Semicolons are required
- ✅ Memory safety without garbage collection
- ✅ Zero-cost abstractions
- ✅ Excellent error messages

Happy Rust coding! 🦀

