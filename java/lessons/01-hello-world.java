/**
 * 01-hello-world.java
 * 
 * A simple Java program to demonstrate the basic structure of a Java application.
 * This program prints "Hello world!" to the console.
 * 
 * File naming convention:
 * - A public class name must match the file name exactly
 * - This file uses a non-public class to allow the "01-" prefix
 * - For production code, use standard naming (see HelloWorld.java)
 * 
 * @author Learn-Lang Tutorial
 * @version 1.0
 */

// Define a class (not public to avoid filename matching requirement)
// Package-private access is sufficient for simple examples
class HelloWorld01 {
    
    /**
     * The main method is the entry point of any Java application.
     * The JVM calls this method when the program starts.
     * 
     * @param args Command-line arguments (not used in this example)
     */
    public static void main(String[] args) {
        // Print "Hello world!" to the console
        // System.out is a PrintStream object for console output
        // println() prints the text and adds a newline at the end
        System.out.println("Hello world!");
        
        // Alternative: use print() without newline
        // System.out.print("Hello world!\n");
        
        // Alternative: use printf() for formatted output
        // System.out.printf("Hello %s!\n", "world");
    }
}

/*
 * Key Java Concepts Demonstrated:
 * 
 * 1. Class Definition: Everything in Java is part of a class
 * 2. public: Access modifier making the class accessible from anywhere
 * 3. static: Method belongs to the class, not an instance
 * 4. void: Method doesn't return any value
 * 5. main(): Entry point method signature required by JVM
 * 6. String[] args: Array to receive command-line arguments
 * 7. System.out.println(): Standard output stream for printing
 * 
 * Compilation and Execution:
 * javac 01-hello-world.java  // Compiles to HelloWorld01.class
 * java HelloWorld01           // Runs the compiled bytecode
 * 
 * Note: For standard Java naming, see HelloWorld.java in the same directory
 */

