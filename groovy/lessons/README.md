# Groovy Hello World Example

A simple Hello World program in Groovy.

## Files

- `01-hello-world.groovy` - Groovy script that prints "Hello world!"

## Prerequisites

### Install Groovy

**macOS (using Homebrew):**
```bash
brew install groovy
```

**macOS (using SDKMAN - Recommended):**
```bash
# Install SDKMAN if not already installed
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"

# Install Groovy
sdk install groovy

# Verify installation
groovy -version
```

**Linux (using SDKMAN):**
```bash
# Install SDKMAN
curl -s "https://get.sdkman.io" | bash
source "$HOME/.bashrc"

# Install Groovy
sdk install groovy
```

**Windows (using Chocolatey):**
```powershell
choco install groovy
```

**Manual Installation:**
1. Download from https://groovy.apache.org/download.html
2. Extract the archive
3. Add `GROOVY_HOME` environment variable
4. Add `$GROOVY_HOME/bin` to PATH

## How to Run

### Method 1: Run as Script (Recommended)
```bash
groovy 01-hello-world.groovy
```

### Method 2: Compile and Run
```bash
# Compile to .class file
groovyc 01-hello-world.groovy

# Run the compiled class
groovy 01-hello-world

# Or using Java
java -cp .:/path/to/groovy-all.jar 01-hello-world
```

### Method 3: Make Executable (Unix/Linux/macOS)
```bash
# Add shebang to first line: #!/usr/bin/env groovy
# Make executable
chmod +x 01-hello-world.groovy

# Run
./01-hello-world.groovy
```

## Expected Output

```
Hello world!
```

## About Groovy

Groovy is a powerful, optionally typed, and dynamic language for the Java platform:
- **Runs on JVM**: Fully compatible with Java
- **Concise Syntax**: Less boilerplate than Java
- **Scripting Support**: Can be run as scripts without compilation
- **Dynamic Features**: Closures, meta-programming, operator overloading

## Key Features Demonstrated

1. **Simple Syntax**: No semicolons required, no main method needed for scripts
2. **Print Statement**: `println` is simpler than Java's `System.out.println`
3. **Script Mode**: Can run directly without class definition
4. **Comments**: Shows standard comment syntax

## Alternative Ways to Print

```groovy
// Simple print
println "Hello world!"

// Java style (also works)
System.out.println("Hello world!")

// Without newline
print "Hello world!\n"

// String interpolation
def message = "world"
println "Hello ${message}!"

// Multiple lines
println """
Hello
world!
"""
```

## Verify Installation

Check if Groovy is installed:
```bash
groovy -version
```

You should see something like:
```
Groovy Version: 5.0.2 JVM: 17.0.x Vendor: Oracle Corporation
```

