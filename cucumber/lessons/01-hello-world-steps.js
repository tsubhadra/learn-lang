// 01-hello-world-steps.js
// Step definitions for the Hello World feature
//JavaScript implementation of the steps
// Contains the actual code that runs when Cucumber executes the feature
// Implements the greeting function and assertions

const { Given, When, Then } = require('@cucumber/cucumber');
const assert = require('assert');

let greeting;
let output;

Given('I have a greeting function', function () {
  // Define a simple greeting function
  greeting = function() {
    return "Hello world!";
  };
});

When('I call the greeting function', function () {
  // Call the greeting function and capture output
  output = greeting();
  console.log(output);
});

Then('I should see {string} printed', function (expectedMessage) {
  // Verify the output matches expected message
  assert.strictEqual(output, expectedMessage);
});

