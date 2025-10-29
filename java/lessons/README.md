# Java Lessons - Hello World Examples

This directory contains Java Hello World examples demonstrating basic Java program structure.

## Files

### Source Files (.java)
- **`HelloWorld.java`** - Standard Java naming convention (recommended)
- **`01-hello-world.java`** - Learning-friendly naming with numeric prefix

### Compiled Files (.class)
- **`HelloWorld.class`** - Compiled bytecode from HelloWorld.java
- **`HelloWorld01.class`** - Compiled bytecode from 01-hello-world.java

## Prerequisites

### Java Development Kit (JDK)

You need JDK 8 or higher installed. Check if Java is installed:

```bash
java -version
javac -version
```

If not installed, install using SDKMAN (recommended):

```bash
# Install SDKMAN
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"

# Install Java
sdk install java 17.0.9-tem

# Verify
java -version
javac -version
```

## How to Compile and Run

### Method 1: Standard HelloWorld.java

```bash
# Compile
javac HelloWorld.java

# Run
java HelloWorld
```

Output:
```
Hello world!
```

### Method 2: With Numeric Prefix (01-hello-world.java)

```bash
# Compile
javac 01-hello-world.java

# Run (note: class name is HelloWorld01, not the filename)
java HelloWorld01
```

Output:
```
Hello world!
```

### Method 3: With SDKMAN (if Java not in PATH)

```bash
# Compile and run in one command
bash -c 'source ~/.sdkman/bin/sdkman-init.sh && javac HelloWorld.java && java HelloWorld'
```

### Method 4: Compile and Run Together

```bash
# Compile and run (using && to chain commands)
javac HelloWorld.java && java HelloWorld

# Or one-liner with error handling
javac HelloWorld.java && java HelloWorld || echo "Compilation or execution failed"
```

## Understanding the Files

### File Size Comparison

```bash
$ ls -lh
-rw-r--r--  2.0K  01-hello-world.java   # Source with extensive comments
-rw-r--r--  2.4K  HelloWorld.java       # Source with extensive comments
-rw-r--r--  426B  HelloWorld.class      # Compiled bytecode
-rw-r--r--  432B  HelloWorld01.class    # Compiled bytecode
```

**Key Points:**
- `.java` files are human-readable source code
- `.class` files are compiled bytecode (binary format)
- `.class` files are much smaller despite containing executable code
- One `.java` file produces one `.class` file per class definition

## Java File Naming Rules

### Important Java Convention

**Rule:** A **public** class name must match the filename exactly.

✅ **Correct:**
```java
// File: HelloWorld.java
public class HelloWorld {
    // ...
}
```

❌ **Incorrect:**
```java
// File: 01-hello-world.java
public class HelloWorld {  // ERROR: Public class in wrong file!
    // ...
}
```

✅ **Workaround for Learning:**
```java
// File: 01-hello-world.java
class HelloWorld01 {  // Not public, so filename can differ
    // ...
}
```

### Why This Matters

1. **Java Compiler Requirement**: Ensures code organization and predictability
2. **Package System**: Helps with classpath resolution
3. **Best Practice**: Makes code easier to find and maintain

### Production vs Learning

- **Production Code**: Use `HelloWorld.java` (standard naming)
- **Learning/Tutorials**: Can use `01-hello-world.java` with non-public classes

## Program Structure Explained

### Anatomy of HelloWorld.java

```java
public class HelloWorld {              // 1. Class declaration
    public static void main(String[] args) {  // 2. Main method
        System.out.println("Hello world!");   // 3. Print statement
    }
}
```

**Breakdown:**

1. **`public class HelloWorld`**
   - `public`: Accessible from anywhere
   - `class`: Defines a class (blueprint for objects)
   - `HelloWorld`: Class name (must match filename)

2. **`public static void main(String[] args)`**
   - `public`: JVM can access from anywhere
   - `static`: Belongs to class (no object needed)
   - `void`: Returns nothing
   - `main`: Special method name JVM looks for
   - `String[] args`: Command-line arguments

3. **`System.out.println("Hello world!")`**
   - `System`: Built-in class
   - `out`: Static field (PrintStream object)
   - `println`: Method to print with newline

## Compilation Process

```
Source Code (.java)
        ↓
    [javac]  ← Java Compiler
        ↓
   Bytecode (.class)
        ↓
      [JVM]  ← Java Virtual Machine
        ↓
   Execution
```

### What Happens During Compilation

```bash
$ javac HelloWorld.java
```

1. **Syntax Check**: Verifies Java syntax is correct
2. **Type Check**: Ensures type safety
3. **Bytecode Generation**: Converts to platform-independent bytecode
4. **Output**: Creates `HelloWorld.class` file

### What Happens During Execution

```bash
$ java HelloWorld
```

1. **Class Loading**: JVM loads HelloWorld.class
2. **Bytecode Verification**: Ensures bytecode is valid
3. **Just-In-Time (JIT) Compilation**: Converts to machine code
4. **Execution**: Runs the main method
5. **Output**: "Hello world!" appears in console

## Viewing Compiled Bytecode (Advanced)

```bash
# Disassemble the class file to see bytecode
javap -c HelloWorld

# Output shows JVM instructions:
# Compiled from "HelloWorld.java"
# public class HelloWorld {
#   public HelloWorld();
#     Code:
#        0: aload_0
#        1: invokespecial #1
#        4: return
#
#   public static void main(java.lang.String[]);
#     Code:
#        0: getstatic     #7
#        3: ldc           #13
#        5: invokevirtual #15
#        8: return
# }
```

## Common Compilation Errors

### 1. Class Name Mismatch

```bash
$ javac HelloWorld.java
error: class HelloWorld is public, should be declared in a file named HelloWorld.java
```

**Solution**: Rename file or make class non-public

### 2. Missing Semicolon

```java
System.out.println("Hello")  // Missing semicolon
```

**Error**: `';' expected`

### 3. Method Not Found

```bash
$ java HelloWorld
Error: Main method not found in class HelloWorld
```

**Solution**: Ensure method signature is exactly `public static void main(String[] args)`

## Alternative Print Methods

### Using print() - No Newline

```java
System.out.print("Hello ");
System.out.print("world!\n");
```

### Using printf() - Formatted Output

```java
System.out.printf("Hello %s!\n", "world");
System.out.printf("Number: %d\n", 42);
System.out.printf("Pi: %.2f\n", 3.14159);
```

### Multiple Lines

```java
System.out.println("Hello");
System.out.println("world!");
```

## Running with Command-Line Arguments

```java
// Modify main to use args
public static void main(String[] args) {
    if (args.length > 0) {
        System.out.println("Hello " + args[0] + "!");
    } else {
        System.out.println("Hello world!");
    }
}
```

```bash
# Compile
javac HelloWorld.java

# Run with argument
java HelloWorld Alice
# Output: Hello Alice!

# Run without argument
java HelloWorld
# Output: Hello world!
```

## Clean Up Compiled Files

```bash
# Remove all .class files
rm *.class

# Or remove specific file
rm HelloWorld.class
```

## Next Steps

After mastering Hello World, explore:

1. **Variables and Data Types** - Storing information
2. **Control Structures** - If/else, loops
3. **Methods** - Code organization and reuse
4. **Classes and Objects** - Object-oriented programming
5. **Arrays and Collections** - Working with multiple values
6. **Exception Handling** - Dealing with errors
7. **File I/O** - Reading and writing files

## Useful Commands

```bash
# Check Java version
java -version
javac -version

# Compile with verbose output
javac -verbose HelloWorld.java

# Run with specific classpath
java -cp . HelloWorld

# Compile multiple files
javac *.java

# Create JAR file
jar cvf hello.jar *.class

# Run JAR file
java -jar hello.jar

# View class file info
javap HelloWorld

# View with bytecode
javap -c HelloWorld
```

## Resources

- **Official Java Tutorials**: https://docs.oracle.com/javase/tutorial/
- **Java API Documentation**: https://docs.oracle.com/en/java/javase/17/docs/api/
- **Project Setup Guide**: See `../docs/initial-setup.txt`

## Troubleshooting

### "java: command not found"

```bash
# Source SDKMAN
source ~/.sdkman/bin/sdkman-init.sh

# Or install Java
sdk install java
```

### "javac: command not found"

You have JRE but not JDK. Install JDK:

```bash
sdk install java 17.0.9-tem
```

### "Could not find or load main class"

1. Check class name matches exactly (case-sensitive)
2. Run from directory containing .class file
3. Don't include .class extension in java command

```bash
# Wrong
java HelloWorld.class

# Correct
java HelloWorld
```

## Summary

- ✅ Java requires proper class/file naming conventions
- ✅ `javac` compiles `.java` to `.class`
- ✅ `java` executes the compiled bytecode
- ✅ Main method signature must be exact
- ✅ Everything in Java is part of a class
- ✅ Two files provided: standard naming and learning-friendly naming

Happy coding! 🎉

