# Instructions on how to set up and run the example
# Explains what each file does

# Cucumber Hello World Example

This is a simple Hello World example using Cucumber for Behavior-Driven Development (BDD).

## Files

- `01-hello-world.feature` - Gherkin feature file with test scenario
- `01-hello-world-steps.js` - Step definitions (implementation)
- `package.json` - Node.js dependencies and scripts

## Setup

1. Install Node.js (if not already installed)
2. Install dependencies:
   ```bash
   npm install
   ```

## Run

Execute the Cucumber test:
```bash
npm test
```

Or run directly with cucumber-js:
```bash
npx cucumber-js 01-hello-world.feature --require 01-hello-world-steps.js
```

## Expected Output

```
Feature: Hello World

  Scenario: Print Hello World message
    Given I have a greeting function
    When I call the greeting function
    Then I should see "Hello world!" printed

1 scenario (1 passed)
3 steps (3 passed)
```

## What This Example Demonstrates

1. **Feature File (.feature)**: Written in Gherkin syntax, describes behavior in plain English
2. **Step Definitions (.js)**: Implements the steps defined in the feature file
3. **Given-When-Then**: BDD pattern for writing tests
4. **Cucumber Execution**: How to run Cucumber tests with Node.js

