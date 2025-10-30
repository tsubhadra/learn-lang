# Rust Calculator

A simple, interactive command-line calculator built with Rust that performs basic arithmetic operations with comprehensive error handling.

## Features

✨ **Operations Supported:**
- Addition (+)
- Subtraction (-)
- Multiplication (*)
- Division (/)

🛡️ **Error Handling:**
- Invalid operation choice detection
- Invalid number input validation
- Division by zero protection
- Input/output error handling

🔄 **User Experience:**
- Interactive menu-driven interface
- Multiple calculations in one session
- Clear, formatted output
- Graceful error messages

## Usage

### Build and Run

```bash
# Navigate to the project directory
cd /Users/suthirumavalavan/learn/learn-lang/rust/calculator

# Build the project
cargo build

# Run the project
cargo run
```

### Example Session

```
===========================================
    Welcome to Rust Calculator!
===========================================

Choose an operation:
  1. Addition (+)
  2. Subtraction (-)
  3. Multiplication (*)
  4. Division (/)
  5. Quit

Enter your choice (1-5): 1
Enter the first number: 15
Enter the second number: 7

===========================================
  RESULT
===========================================
  Operation: Addition
  First Number: 15
  Second Number: 7
  Calculation: 15 + 7 = 22
===========================================

-------------------------------------------

Choose an operation:
  1. Addition (+)
  2. Subtraction (-)
  3. Multiplication (*)
  4. Division (/)
  5. Quit

Enter your choice (1-5): 4
Enter the first number: 100
Enter the second number: 5

===========================================
  RESULT
===========================================
  Operation: Division
  First Number: 100
  Second Number: 5
  Calculation: 100 / 5 = 20
===========================================
```

## Testing

The project includes comprehensive unit tests for all operations:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_division_by_zero
```

### Test Coverage

- ✅ Addition with positive, negative, and zero values
- ✅ Subtraction with various scenarios
- ✅ Multiplication including zero cases
- ✅ Division with proper results
- ✅ Division by zero error handling
- ✅ String capitalization utility

## Code Structure

```
calculator/
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── src/
    └── main.rs         # Main application code
```

### Key Functions

| Function | Purpose |
|----------|---------|
| `main()` | Entry point, manages calculator loop |
| `get_operation()` | Gets operation choice from user with validation |
| `get_number()` | Gets and validates numeric input |
| `calculate()` | Performs the arithmetic operation |
| `print_result()` | Displays formatted result |
| `capitalize()` | Helper for string formatting |

## Error Handling Techniques

### 1. **Result Type Usage**
All functions that can fail return `Result<T, String>` for proper error propagation.

```rust
fn get_number(position: &str) -> Result<f64, String> {
    // ... implementation
}
```

### 2. **Input Validation**
- Operation choice validated against allowed options
- Numeric input parsed with error handling
- Clear error messages for invalid input

### 3. **Division by Zero Check**
```rust
if num2 == 0.0 {
    Err("Cannot divide by zero!".to_string())
} else {
    Ok(num1 / num2)
}
```

### 4. **IO Error Handling**
All I/O operations use `.map_err()` to convert errors to user-friendly messages.

### 5. **Graceful Recovery**
Invalid input doesn't crash the program - the loop continues, allowing users to try again.

## Input Flexibility

The calculator accepts multiple input formats for operations:

| Operation | Accepted Inputs |
|-----------|----------------|
| Addition | `1`, `+`, `add`, `addition` |
| Subtraction | `2`, `-`, `sub`, `subtract`, `subtraction` |
| Multiplication | `3`, `*`, `mul`, `multiply`, `multiplication` |
| Division | `4`, `/`, `div`, `divide`, `division` |
| Quit | `5`, `q`, `quit`, `exit` |

## Learning Objectives

This project demonstrates:

1. **User Input Handling**: Reading and validating user input with `std::io`
2. **Error Handling**: Proper use of `Result` type and `?` operator
3. **Pattern Matching**: Using `match` for control flow
4. **String Formatting**: Using `println!` macros for output
5. **Function Organization**: Breaking code into logical, testable functions
6. **Unit Testing**: Writing and running tests with `#[test]`
7. **Documentation**: Clear comments and function documentation

## Advanced Features

### Continuous Operation
The calculator runs in a loop, allowing multiple calculations without restarting.

### Comprehensive Testing
All arithmetic operations and error cases are covered by unit tests.

### User-Friendly Messages
- Clear prompts for input
- Formatted output for results
- Helpful error messages

## Future Enhancements

Potential improvements:
- [ ] More operations (power, square root, modulo)
- [ ] History of calculations
- [ ] Save/load calculation history
- [ ] Scientific notation support
- [ ] Complex number operations
- [ ] Configurable precision

## Dependencies

This project uses only Rust standard library, no external crates required!

## License

Educational project for learning Rust programming.

## Author

Learn-Lang Tutorial Series

