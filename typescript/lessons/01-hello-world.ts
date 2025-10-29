/**
 * 01-hello-world.ts
 * 
 * A simple TypeScript program to demonstrate the basic structure of a TypeScript application.
 * This program prints "Hello world!" to the console.
 * 
 * TypeScript is a typed superset of JavaScript that compiles to plain JavaScript.
 * It adds optional static typing, classes, and interfaces to JavaScript.
 * 
 * @author Learn-Lang Tutorial
 * @version 1.0
 */

// Main execution starts here
// TypeScript is similar to JavaScript but with type annotations

// Method 1: Using console.log() - most common
console.log("Hello world!");

// Alternative methods with TypeScript features:

/**
 * Example 1: With explicit string type
 */
const message: string = "Hello world!";
// console.log(message);

/**
 * Example 2: Template literals (same as JavaScript)
 */
const name: string = "world";
// console.log(`Hello ${name}!`);

/**
 * Example 3: Function with type annotations
 * 
 * @param greeting - The greeting message
 * @returns void (returns nothing)
 */
function greet(greeting: string): void {
    console.log(greeting);
}
// greet("Hello world!");

/**
 * Example 4: Arrow function with type annotations
 */
const greetArrow = (greeting: string): void => {
    console.log(greeting);
};
// greetArrow("Hello world!");

/**
 * Example 5: Function that returns a string
 * 
 * @returns The greeting message
 */
function getGreeting(): string {
    return "Hello world!";
}
// console.log(getGreeting());

/**
 * Example 6: Using interfaces (TypeScript feature)
 */
interface Greeting {
    message: string;
    recipient: string;
}

const greeting: Greeting = {
    message: "Hello",
    recipient: "world"
};
// console.log(`${greeting.message} ${greeting.recipient}!`);

/**
 * Example 7: Using type aliases
 */
type Message = string;
const msg: Message = "Hello world!";
// console.log(msg);

/**
 * Example 8: Using class (TypeScript/ES6)
 */
class Greeter {
    private message: string;
    
    constructor(message: string) {
        this.message = message;
    }
    
    greet(): void {
        console.log(this.message);
    }
}
// const greeter = new Greeter("Hello world!");
// greeter.greet();

/**
 * Example 9: Using enum (TypeScript feature)
 */
enum GreetingType {
    Formal = "Good day",
    Informal = "Hello",
    Casual = "Hey"
}
// console.log(`${GreetingType.Informal} world!`);

/**
 * Example 10: Generic function (TypeScript feature)
 */
function print<T>(value: T): void {
    console.log(value);
}
// print<string>("Hello world!");

/*
 * Key TypeScript Concepts Demonstrated:
 * 
 * 1. Type Annotations - Explicitly declare variable types
 * 2. Interfaces - Define object shapes and contracts
 * 3. Type Aliases - Create custom type names
 * 4. Classes - Object-oriented programming
 * 5. Enums - Named constants
 * 6. Generics - Type-safe reusable code
 * 7. Access Modifiers - public, private, protected
 * 8. Return Type Annotations - Specify function return types
 * 9. Void Type - For functions that don't return values
 * 10. Compile-time Type Checking - Catch errors before runtime
 * 
 * Compilation and Execution:
 * 
 *   # Compile TypeScript to JavaScript
 *   tsc 01-hello-world.ts
 * 
 *   # This creates 01-hello-world.js
 *   # Then run with Node.js
 *   node 01-hello-world.js
 * 
 *   # Or compile and run in one step using ts-node
 *   ts-node 01-hello-world.ts
 * 
 *   # Or using tsx (faster)
 *   tsx 01-hello-world.ts
 * 
 * Expected Output:
 *   Hello world!
 * 
 * File Extensions:
 *   .ts - TypeScript source code (human-readable)
 *   .js - Compiled JavaScript (human-readable)
 *   .d.ts - TypeScript declaration files
 * 
 * TypeScript vs JavaScript:
 * - TypeScript has static typing, JavaScript is dynamically typed
 * - TypeScript catches errors at compile-time
 * - TypeScript has interfaces, enums, generics
 * - TypeScript compiles to JavaScript
 * - TypeScript is a superset of JavaScript (all JS is valid TS)
 */

// Optional: Export for use as a module
export {};

// Optional: If this file is run directly (requires @types/node)
// if (require.main === module) {
//     // Additional code to run when executed directly
// }

