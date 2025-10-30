# Calculator Examples

## Quick Start

```bash
cd /Users/suthirumavalavan/learn/learn-lang/rust/calculator
cargo run --release
```

## Example Usage Sessions

### Example 1: Addition
```
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
```

### Example 2: Subtraction
```
Enter your choice (1-5): 2
Enter the first number: 100
Enter the second number: 45

===========================================
  RESULT
===========================================
  Operation: Subtraction
  First Number: 100
  Second Number: 45
  Calculation: 100 - 45 = 55
===========================================
```

### Example 3: Multiplication
```
Enter your choice (1-5): 3
Enter the first number: 12
Enter the second number: 8

===========================================
  RESULT
===========================================
  Operation: Multiplication
  First Number: 12
  Second Number: 8
  Calculation: 12 * 8 = 96
===========================================
```

### Example 4: Division
```
Enter your choice (1-5): 4
Enter the first number: 50
Enter the second number: 2

===========================================
  RESULT
===========================================
  Operation: Division
  First Number: 50
  Second Number: 2
  Calculation: 50 / 2 = 25
===========================================
```

### Example 5: Division with Decimals
```
Enter your choice (1-5): 4
Enter the first number: 22
Enter the second number: 7

===========================================
  RESULT
===========================================
  Operation: Division
  First Number: 22
  Second Number: 7
  Calculation: 22 / 7 = 3.142857142857143
===========================================
```

## Error Handling Examples

### Example 6: Division by Zero (Error Handling)
```
Enter your choice (1-5): 4
Enter the first number: 100
Enter the second number: 0

Error: Cannot divide by zero!
```

### Example 7: Invalid Number Input
```
Enter your choice (1-5): 1
Enter the first number: abc
Error: 'abc' is not a valid number. Please enter a numeric value.
```

### Example 8: Invalid Operation Choice
```
Enter your choice (1-5): 9
Error: Invalid choice: '9'. Please choose 1-5.
```

### Example 9: Negative Numbers
```
Enter your choice (1-5): 1
Enter the first number: -15
Enter the second number: 25

===========================================
  RESULT
===========================================
  Operation: Addition
  First Number: -15
  Second Number: 25
  Calculation: -15 + 25 = 10
===========================================
```

## Alternative Input Methods

### Using Operation Names
```
Enter your choice (1-5): add
Enter the first number: 5
Enter the second number: 3
Result: 8
```

### Using Symbols
```
Enter your choice (1-5): +
Enter the first number: 10
Enter the second number: 20
Result: 30
```

### Quitting
```
Enter your choice (1-5): q
Thank you for using Rust Calculator. Goodbye!
```

or

```
Enter your choice (1-5): quit
Thank you for using Rust Calculator. Goodbye!
```

or

```
Enter your choice (1-5): 5
Thank you for using Rust Calculator. Goodbye!
```

## Testing

### Run All Tests
```bash
cargo test
```

Expected output:
```
running 6 tests
test tests::test_addition ... ok
test tests::test_capitalize ... ok
test tests::test_division ... ok
test tests::test_division_by_zero ... ok
test tests::test_multiplication ... ok
test tests::test_subtraction ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Run Specific Test
```bash
cargo test test_division_by_zero
```

### Run Tests with Verbose Output
```bash
cargo test -- --nocapture --test-threads=1
```

## Key Features Demonstrated

✅ **User Input Validation**
- Non-numeric input rejected with clear error message
- Invalid operation choice handled gracefully

✅ **Mathematical Operations**
- Addition, subtraction, multiplication, division
- Supports positive, negative, and decimal numbers

✅ **Error Handling**
- Division by zero prevented
- Invalid input doesn't crash the program
- Clear error messages guide the user

✅ **Continuous Operation**
- Multiple calculations in one session
- Errors allow retry without restarting

✅ **Flexible Input**
- Numbers: `1`, `2`, `3`, `4`, `5`
- Symbols: `+`, `-`, `*`, `/`
- Words: `add`, `subtract`, `multiply`, `divide`
- Short words: `sub`, `mul`, `div`
- Quit: `q`, `quit`, `exit`, `5`

## Performance

The calculator is built in release mode for optimal performance:

```bash
cargo build --release
./target/release/calculator
```

Release mode provides:
- Optimized code execution
- Smaller binary size
- Faster startup time

