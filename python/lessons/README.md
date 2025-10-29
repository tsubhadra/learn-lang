# Python Lessons - Hello World Example

This directory contains a Python Hello World example demonstrating basic Python program structure.

## Files

### Source Files
- **`01-hello-world.py`** - Python script that prints "Hello world!"

### Compiled Files (Auto-generated)
- **`__pycache__/01-hello-world.cpython-39.pyc`** - Compiled bytecode (created automatically)

## Prerequisites

### Python Installation

**Check if Python is installed:**
```bash
python3 --version
# or
python --version
```

**If not installed:**

**macOS:**
```bash
# Using Homebrew
brew install python3

# Or download from python.org
# Visit: https://www.python.org/downloads/
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt update
sudo apt install python3 python3-pip
```

**Linux (using pyenv - Recommended for version management):**
```bash
# Install pyenv
curl https://pyenv.run | bash

# Install Python
pyenv install 3.12.0
pyenv global 3.12.0
```

**Windows:**
```powershell
# Using Chocolatey
choco install python

# Or download installer from python.org
# Important: Check "Add Python to PATH" during installation
```

## How to Run Python Programs

### Method 1: Direct Execution (Most Common)

```bash
# Run with python3
python3 01-hello-world.py

# Or with python (if python3 is default)
python 01-hello-world.py
```

**Output:**
```
Hello world!
```

### Method 2: Make Executable (Unix/Linux/macOS)

```bash
# Make the script executable
chmod +x 01-hello-world.py

# Run directly
./01-hello-world.py
```

**Note:** Requires shebang `#!/usr/bin/env python3` at the top of the file.

### Method 3: Interactive Python (REPL)

```bash
# Start Python interactive shell
python3

# Then type:
>>> print("Hello world!")
Hello world!
>>> exit()
```

### Method 4: Compiled Bytecode

```bash
# Compile to bytecode
python3 -m py_compile 01-hello-world.py

# This creates: __pycache__/01-hello-world.cpython-39.pyc

# Run compiled bytecode
python3 __pycache__/01-hello-world.cpython-*.pyc
```

### Method 5: Using python -c (One-liner)

```bash
python3 -c "print('Hello world!')"
```

## Understanding Python Files

### File Extensions

```
01-hello-world.py          # Source code (human-readable)
01-hello-world.pyc         # Bytecode (compiled)
__pycache__/               # Directory for bytecode cache
```

### Python vs Other Languages

**Java (compiled):**
```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}
```
```bash
javac HelloWorld.java  # Explicit compilation required
java HelloWorld        # Run
```

**Python (interpreted/compiled):**
```python
print("Hello world!")
```
```bash
python hello.py  # Run directly (compiles to bytecode automatically)
```

**Key Differences:**
- ❌ No explicit compilation step required
- ❌ No class or main() function required (but recommended)
- ❌ No type declarations needed
- ✅ Direct execution
- ✅ Simple, readable syntax
- ✅ Automatic bytecode caching

## Program Structure Explained

### Minimal Python Program

```python
# Simplest possible program
print("Hello world!")
```

That's it! Run with: `python3 hello.py`

### Recommended Structure (What We Use)

```python
#!/usr/bin/env python3
"""Module docstring - describes what this file does."""

def main():
    """Main function docstring."""
    print("Hello world!")

if __name__ == "__main__":
    main()
```

**Why this structure?**

1. **Shebang (`#!/usr/bin/env python3`)**
   - Makes script executable on Unix systems
   - Specifies Python interpreter

2. **Module Docstring**
   - Documents the file's purpose
   - Accessible via `__doc__`
   - Used by `help()` function

3. **main() Function**
   - Organizes code
   - Allows importing without execution
   - Makes testing easier

4. **if __name__ == "__main__"**
   - Standard Python idiom
   - Code runs only when script is executed directly
   - Doesn't run when imported as a module

### Anatomy of Python Code

```python
#!/usr/bin/env python3          # 1. Shebang (optional, for Unix)
"""Docstring"""                 # 2. Module documentation

def main():                     # 3. Function definition
    """Function docstring"""    # 4. Function documentation
    print("Hello world!")       # 5. Indented code block

if __name__ == "__main__":      # 6. Execution guard
    main()                      # 7. Function call
```

## Python Compilation Process

### Automatic Compilation

```
Source Code (.py)
        ↓
    [Python]  ← Interpreter
        ↓
   Bytecode (.pyc) ← Cached automatically
        ↓
   [Python VM]  ← Executes bytecode
        ↓
   "Hello world!"  ← Output
```

**When bytecode is created:**
- When module is imported
- When using `py_compile` module
- Stored in `__pycache__` directory
- Speeds up subsequent runs

**Bytecode characteristics:**
- Platform-independent
- Not human-readable
- Faster to load than source
- Automatically regenerated if source changes

## Different Ways to Print in Python

### Basic Print

```python
# Simple print
print("Hello world!")

# Multiple arguments (space-separated)
print("Hello", "world!")

# Without newline
print("Hello world!", end="")

# With custom separator
print("Hello", "world", sep="-")  # Hello-world
```

### String Formatting

```python
# 1. f-strings (Python 3.6+) - RECOMMENDED
name = "world"
print(f"Hello {name}!")

# 2. str.format() method
print("Hello {}!".format("world"))
print("Hello {name}!".format(name="world"))

# 3. % formatting (old style)
print("Hello %s!" % "world")

# 4. String concatenation
print("Hello" + " " + "world!")
```

### Advanced Printing

```python
import sys

# Print to stderr
print("Hello world!", file=sys.stderr)

# Print to file
with open("output.txt", "w") as f:
    print("Hello world!", file=f)

# Using write() method
sys.stdout.write("Hello world!\n")
```

## Python Features Demonstrated

### 1. No Type Declarations

```python
# Python (dynamic typing)
message = "Hello world!"
print(message)

# vs Java (static typing)
# String message = "Hello world!";
# System.out.println(message);
```

### 2. Indentation-Based Blocks

```python
# Python uses indentation (4 spaces)
def greet():
    message = "Hello"
    print(message)

# No braces needed!
```

### 3. Functions are First-Class

```python
def greet():
    return "Hello world!"

# Assign function to variable
greeting = greet
print(greeting())

# Pass function as argument
def call_func(func):
    print(func())

call_func(greet)
```

### 4. Multiple Ways to Define Functions

```python
# Regular function
def greet():
    print("Hello world!")

# Lambda (anonymous function)
greet_lambda = lambda: print("Hello world!")

# One-liner function
def greet_one_line(): print("Hello world!")
```

## Common Python Patterns

### 1. Main Function Pattern

```python
def main():
    print("Hello world!")

if __name__ == "__main__":
    main()
```

### 2. With Command-Line Arguments

```python
import sys

def main():
    if len(sys.argv) > 1:
        print(f"Hello {sys.argv[1]}!")
    else:
        print("Hello world!")

if __name__ == "__main__":
    main()
```

```bash
python3 hello.py Alice  # Output: Hello Alice!
```

### 3. Using argparse (Better CLI)

```python
import argparse

def main():
    parser = argparse.ArgumentParser(description='Greet someone')
    parser.add_argument('name', nargs='?', default='world',
                       help='Name to greet')
    args = parser.parse_args()
    print(f"Hello {args.name}!")

if __name__ == "__main__":
    main()
```

### 4. As a Module

```python
# hello.py
def greet(name="world"):
    return f"Hello {name}!"

# another_file.py
from hello import greet
print(greet("Alice"))
```

## File Sizes and Bytecode

```bash
$ ls -lh
-rwxr-xr-x  229B  01-hello-world.py          # Source
-rw-r--r--  450B  __pycache__/...pyc         # Bytecode (auto-generated)
```

**Bytecode characteristics:**
- Larger than source (contains metadata)
- Binary format (not human-readable)
- Platform-independent
- Python version-specific (cpython-39, cpython-310, etc.)

## Viewing Bytecode (Advanced)

```bash
# Disassemble bytecode to see instructions
python3 -m dis 01-hello-world.py

# Output shows Python VM instructions:
#   1           0 LOAD_CONST               0 ('Hello world!')
#               2 CALL_FUNCTION            1
#               4 POP_TOP
#               6 LOAD_CONST               1 (None)
#               8 RETURN_VALUE
```

## Virtual Environments

For real projects, use virtual environments:

```bash
# Create virtual environment
python3 -m venv venv

# Activate
source venv/bin/activate  # Unix/macOS
venv\Scripts\activate     # Windows

# Now python points to venv python
python --version

# Deactivate
deactivate
```

## Common Errors and Solutions

### 1. "python: command not found"

**Solution:**
```bash
# Try python3 instead
python3 01-hello-world.py

# Or install Python
brew install python3  # macOS
sudo apt install python3  # Linux
```

### 2. "Permission denied: ./01-hello-world.py"

**Solution:**
```bash
chmod +x 01-hello-world.py
```

### 3. "SyntaxError: invalid syntax"

**Problem:** Code syntax error

**Check:**
- Indentation (must be consistent)
- Missing colons after def, if, for, etc.
- Mismatched quotes or parentheses

### 4. "IndentationError"

**Problem:** Inconsistent indentation

**Solution:**
- Use 4 spaces (PEP 8 standard)
- Don't mix tabs and spaces
- Configure editor to use spaces

### 5. Wrong Python version

```bash
# Check version
python3 --version

# Use specific version
python3.12 01-hello-world.py

# Or with pyenv
pyenv versions
pyenv local 3.12.0
```

## Python Versions

### Current Versions
- **Python 3.12** - Latest stable (2023)
- **Python 3.11** - Fast and stable
- **Python 3.10** - Widely used
- **Python 3.9** - Still supported
- **Python 2.7** - Deprecated (EOL 2020) ❌

### Recommended Version
Use Python 3.10+ for new projects.

### Check Your Version
```bash
python3 --version
python --version
```

## Useful Commands

```bash
# Run script
python3 script.py

# Interactive shell (REPL)
python3

# Execute string
python3 -c "print('Hello')"

# Run module as script
python3 -m module_name

# Check syntax without running
python3 -m py_compile script.py

# Optimize bytecode (remove assert, __debug__)
python3 -O script.py

# Get help
python3 -h
python3 --help

# Install packages
pip3 install package_name

# Create virtual environment
python3 -m venv myenv

# Format code (requires black)
black script.py

# Lint code (requires pylint)
pylint script.py
```

## Next Steps

After mastering Hello World, explore:

1. **Variables and Data Types**
   - Numbers, strings, lists, dictionaries
   - Type hints (Python 3.5+)

2. **Control Structures**
   - if/elif/else
   - for loops, while loops
   - List comprehensions

3. **Functions**
   - Parameters and return values
   - *args and **kwargs
   - Decorators

4. **Classes and Objects**
   - Object-oriented programming
   - Inheritance
   - Magic methods

5. **Modules and Packages**
   - Importing
   - Creating packages
   - __init__.py files

6. **File I/O**
   - Reading and writing files
   - Context managers (with statement)

7. **Exception Handling**
   - try/except blocks
   - Custom exceptions

8. **Standard Library**
   - os, sys, pathlib
   - json, csv, datetime
   - collections, itertools

## Resources

- **Python Official Docs**: https://docs.python.org/3/
- **Python Tutorial**: https://docs.python.org/3/tutorial/
- **Real Python**: https://realpython.com/
- **Python.org**: https://www.python.org/
- **PEP 8 Style Guide**: https://pep8.org/
- **Project Setup Guide**: See `../docs/initial-setup.txt`

## Tips

1. **Use Python 3**: Python 2 is deprecated
2. **Follow PEP 8**: Python style guide
3. **Use virtual environments**: For every project
4. **Type hints**: Use for better code documentation
5. **Docstrings**: Document all functions and classes
6. **List comprehensions**: More Pythonic than loops
7. **f-strings**: Modern string formatting
8. **Context managers**: Use `with` for file handling
9. **Pythonic code**: Learn idioms and best practices
10. **Testing**: Use pytest or unittest

## Troubleshooting

### Script runs but no output

Check if main() is being called:
```python
if __name__ == "__main__":
    main()  # This line must be present
```

### Import errors

```bash
# Check Python path
python3 -c "import sys; print('\n'.join(sys.path))"

# Install missing packages
pip3 install package_name
```

### Different behavior in REPL vs script

Some code behaves differently in interactive mode. Always test scripts by running them as scripts.

## Summary

- ✅ Python is interpreted (no explicit compilation)
- ✅ Simple syntax - minimal boilerplate
- ✅ `print()` function for output
- ✅ Indentation-based blocks
- ✅ Dynamic typing
- ✅ Automatic bytecode caching
- ✅ Cross-platform
- ✅ Extensive standard library
- ✅ Great for beginners and experts

Happy Python coding! 🐍

