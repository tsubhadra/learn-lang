/**
 * 01-hello-world.js
 * 
 * A simple JavaScript program to demonstrate the basic structure of a JavaScript application.
 * This program prints "Hello world!" to the console.
 * 
 * JavaScript is an interpreted language (no compilation needed).
 * It can run in:
 * - Web browsers (client-side)
 * - Node.js (server-side)
 * - Deno (modern JavaScript runtime)
 * 
 * @author Learn-Lang Tutorial
 * @version 1.0
 */

// Main execution starts here
// In JavaScript, there's no required main() function like Java
// Code executes from top to bottom

// Method 1: Using console.log() - most common
console.log("Hello world!");

/* 
 * Alternative printing methods:
 * 
 * 1. console.log() - with newline (most common)
 * console.log("Hello world!");
 * 
 * 2. console.log() - multiple arguments
 * console.log("Hello", "world!");
 * 
 * 3. Template literals (ES6+) - modern JavaScript
 * const message = "world";
 * console.log(`Hello ${message}!`);
 * 
 * 4. String concatenation
 * console.log("Hello" + " " + "world!");
 * 
 * 5. console.info() - informational message (same as log)
 * console.info("Hello world!");
 * 
 * 6. console.error() - error message (prints to stderr)
 * console.error("Hello world!");
 * 
 * 7. console.warn() - warning message
 * console.warn("Hello world!");
 * 
 * 8. process.stdout.write() - Node.js specific (no automatic newline)
 * process.stdout.write("Hello world!\n");
 */

/*
 * Key JavaScript Concepts Demonstrated:
 * 
 * 1. No compilation needed - interpreted language
 * 2. No main() function required - code runs top-to-bottom
 * 3. console.log() - standard way to print output
 * 4. Semicolons are optional (but recommended)
 * 5. Both single-line (//) and multi-line (/* *\/) comments work
 * 6. Flexible syntax - less strict than Java/C++
 * 
 * Execution:
 * node 01-hello-world.js      // Run with Node.js
 * 
 * In browser:
 * <script src="01-hello-world.js"></script>
 * // Open browser console (F12) to see output
 * 
 * Expected Output:
 * Hello world!
 */

// Optional: Export for use as a module (Node.js)
// module.exports = { greeting: "Hello world!" };

// Optional: ES6 module export
// export default function() { console.log("Hello world!"); }

