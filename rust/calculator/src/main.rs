use std::io::{self, Write};

/// Main entry point for the calculator application
fn main() {
    println!("===========================================");
    println!("    Welcome to Rust Calculator!");
    println!("===========================================\n");

    // Main calculator loop - allows multiple calculations
    loop {
        // Get the operation from user
        let operation = match get_operation() {
            Ok(op) => op,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // Check if user wants to quit
        if operation == "quit" {
            println!("\nThank you for using Rust Calculator. Goodbye!");
            break;
        }

        // Get the first number
        let num1 = match get_number("first") {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // Get the second number
        let num2 = match get_number("second") {
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            }
        };

        // Perform the calculation
        match calculate(&operation, num1, num2) {
            Ok(result) => {
                // Display the result in a formatted way
                print_result(&operation, num1, num2, result);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }

        println!("\n-------------------------------------------\n");
    }
}

/// Gets the operation choice from the user
/// Returns: Result<String, String> - operation name or error message
fn get_operation() -> Result<String, String> {
    println!("Choose an operation:");
    println!("  1. Addition (+)");
    println!("  2. Subtraction (-)");
    println!("  3. Multiplication (*)");
    println!("  4. Division (/)");
    println!("  5. Quit");
    print!("\nEnter your choice (1-5): ");
    
    // Flush stdout to ensure prompt appears
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {}", e))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    let choice = input.trim();

    // Match the choice to an operation
    match choice {
        "1" | "+" | "add" | "addition" => Ok("addition".to_string()),
        "2" | "-" | "sub" | "subtract" | "subtraction" => Ok("subtraction".to_string()),
        "3" | "*" | "mul" | "multiply" | "multiplication" => Ok("multiplication".to_string()),
        "4" | "/" | "div" | "divide" | "division" => Ok("division".to_string()),
        "5" | "q" | "quit" | "exit" => Ok("quit".to_string()),
        _ => Err(format!("Invalid choice: '{}'. Please choose 1-5.", choice)),
    }
}

/// Gets a number from the user
/// Parameters:
///   - position: "first" or "second" to indicate which number we're asking for
/// Returns: Result<f64, String> - the number or error message
fn get_number(position: &str) -> Result<f64, String> {
    print!("Enter the {} number: ", position);
    
    // Flush stdout to ensure prompt appears
    io::stdout()
        .flush()
        .map_err(|e| format!("Failed to flush stdout: {}", e))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    // Parse the input as f64
    input
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("'{}' is not a valid number. Please enter a numeric value.", input.trim()))
}

/// Performs the calculation based on the operation
/// Parameters:
///   - operation: the operation to perform
///   - num1: first number
///   - num2: second number
/// Returns: Result<f64, String> - the result or error message
fn calculate(operation: &str, num1: f64, num2: f64) -> Result<f64, String> {
    match operation {
        "addition" => Ok(num1 + num2),
        "subtraction" => Ok(num1 - num2),
        "multiplication" => Ok(num1 * num2),
        "division" => {
            // Check for division by zero
            if num2 == 0.0 {
                Err("Cannot divide by zero!".to_string())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => Err(format!("Unknown operation: {}", operation)),
    }
}

/// Prints the result in a formatted way
/// Parameters:
///   - operation: the operation performed
///   - num1: first number
///   - num2: second number
///   - result: the calculated result
fn print_result(operation: &str, num1: f64, num2: f64, result: f64) {
    // Get the operation symbol
    let symbol = match operation {
        "addition" => "+",
        "subtraction" => "-",
        "multiplication" => "*",
        "division" => "/",
        _ => "?",
    };

    println!("\n===========================================");
    println!("  RESULT");
    println!("===========================================");
    println!("  Operation: {}", capitalize(operation));
    println!("  First Number: {}", num1);
    println!("  Second Number: {}", num2);
    println!("  Calculation: {} {} {} = {}", num1, symbol, num2, result);
    println!("===========================================");
}

/// Capitalizes the first letter of a string
/// Parameters:
///   - s: the string to capitalize
/// Returns: capitalized string
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(calculate("addition", 5.0, 3.0).unwrap(), 8.0);
        assert_eq!(calculate("addition", -5.0, 3.0).unwrap(), -2.0);
        assert_eq!(calculate("addition", 0.0, 0.0).unwrap(), 0.0);
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calculate("subtraction", 5.0, 3.0).unwrap(), 2.0);
        assert_eq!(calculate("subtraction", 3.0, 5.0).unwrap(), -2.0);
        assert_eq!(calculate("subtraction", 0.0, 0.0).unwrap(), 0.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calculate("multiplication", 5.0, 3.0).unwrap(), 15.0);
        assert_eq!(calculate("multiplication", -5.0, 3.0).unwrap(), -15.0);
        assert_eq!(calculate("multiplication", 0.0, 5.0).unwrap(), 0.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(calculate("division", 10.0, 2.0).unwrap(), 5.0);
        assert_eq!(calculate("division", -10.0, 2.0).unwrap(), -5.0);
        assert_eq!(calculate("division", 7.0, 2.0).unwrap(), 3.5);
    }

    #[test]
    fn test_division_by_zero() {
        let result = calculate("division", 10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot divide by zero!");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("addition"), "Addition");
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
    }
}
