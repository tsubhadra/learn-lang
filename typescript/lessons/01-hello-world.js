"use strict";
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
Object.defineProperty(exports, "__esModule", { value: true });
// Main execution starts here
// TypeScript is similar to JavaScript but with type annotations
// Method 1: Using console.log() - most common
console.log("Hello world!");
// Alternative methods with TypeScript features:
/**
 * Example 1: With explicit string type
 */
var message = "Hello world!";
// console.log(message);
/**
 * Example 2: Template literals (same as JavaScript)
 */
var name = "world";
// console.log(`Hello ${name}!`);
/**
 * Example 3: Function with type annotations
 *
 * @param greeting - The greeting message
 * @returns void (returns nothing)
 */
function greet(greeting) {
    console.log(greeting);
}
// greet("Hello world!");
/**
 * Example 4: Arrow function with type annotations
 */
var greetArrow = function (greeting) {
    console.log(greeting);
};
// greetArrow("Hello world!");
/**
 * Example 5: Function that returns a string
 *
 * @returns The greeting message
 */
function getGreeting() {
    return "Hello world!";
}
var greeting = {
    message: "Hello",
    recipient: "world"
};
var msg = "Hello world!";
// console.log(msg);
/**
 * Example 8: Using class (TypeScript/ES6)
 */
var Greeter = /** @class */ (function () {
    function Greeter(message) {
        this.message = message;
    }
    Greeter.prototype.greet = function () {
        console.log(this.message);
    };
    return Greeter;
}());
// const greeter = new Greeter("Hello world!");
// greeter.greet();
/**
 * Example 9: Using enum (TypeScript feature)
 */
var GreetingType;
(function (GreetingType) {
    GreetingType["Formal"] = "Good day";
    GreetingType["Informal"] = "Hello";
    GreetingType["Casual"] = "Hey";
})(GreetingType || (GreetingType = {}));
// console.log(`${GreetingType.Informal} world!`);
/**
 * Example 10: Generic function (TypeScript feature)
 */
function print(value) {
    console.log(value);
}
// Optional: If this file is run directly (requires @types/node)
// if (require.main === module) {
//     // Additional code to run when executed directly
// }
