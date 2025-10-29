/**
 * HelloWorld.java
 * 
 * A simple Java program to demonstrate the basic structure of a Java application.
 * This program prints "Hello world!" to the console.
 * 
 * Important: In Java, the public class name must match the filename exactly.
 * 
 * @author Learn-Lang Tutorial
 * @version 1.0
 */

// Define a public class - must match the filename (HelloWorld.java)
public class HelloWorld {
    
    /**
     * The main method is the entry point of any Java application.
     * The JVM calls this method when the program starts.
     * 
     * Method signature breakdown:
     * - public: Accessible from anywhere
     * - static: Belongs to class, not instance (no object creation needed)
     * - void: Doesn't return any value
     * - main: Special method name recognized by JVM
     * - String[] args: Array to receive command-line arguments
     * 
     * @param args Command-line arguments (not used in this example)
     */
    public static void main(String[] args) {
        // Print "Hello world!" to the console
        // System.out is a PrintStream object for console output
        // println() prints the text and adds a newline at the end
        System.out.println("Hello world!");
        
        /*
         * Alternative printing methods:
         * 
         * 1. print() - without newline:
         * System.out.print("Hello world!\n");
         * 
         * 2. printf() - formatted output:
         * System.out.printf("Hello %s!\n", "world");
         * 
         * 3. Multiple lines:
         * System.out.println("Hello");
         * System.out.println("world!");
         */
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
 * File Naming Rules:
 * - Public class name MUST match filename
 * - File extension must be .java
 * - Compiled file will have .class extension
 * 
 * Compilation and Execution:
 * $ javac HelloWorld.java     // Compiles to HelloWorld.class
 * $ java HelloWorld            // Runs the compiled bytecode
 * 
 * Expected Output:
 * Hello world!
 */
