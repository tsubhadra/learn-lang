package com.example

import spock.lang.Specification

/**
 * 01-HelloWorldSpec.groovy
 *
 * A simple Spock specification to demonstrate the basic structure of Spock tests.
 * This specification prints "Hello world!" when executed.
 *
 * Spock is a testing and specification framework for Java and Groovy applications.
 * It combines the best features of JUnit, Mockito, and other testing frameworks
 * with a clean, expressive syntax inspired by BDD (Behavior-Driven Development).
 *
 * Author: Learn-Lang Tutorial
 * Version: 1.0
 */
class HelloWorldSpec extends Specification {
    
    /**
     * Spock feature method - describes a specific behavior or scenario
     * Feature methods are the heart of Spock specifications
     * 
     * Method name is a string literal - can contain spaces and special characters
     * This makes tests more readable and self-documenting
     */
    def "prints Hello world message"() {
        // given: Setup phase - prepare test preconditions
        // In BDD terms: "Given some initial context"
        given: "a greeting message"
        def message = "Hello world!"
        
        // when: Stimulus phase - perform the action being tested
        // In BDD terms: "When an event occurs"
        when: "we print the message"
        println(message)  // Print to console
        
        // then: Response phase - verify expected outcomes
        // In BDD terms: "Then ensure some outcomes"
        then: "the message should be defined and not empty"
        message != null
        message.length() > 0
        message == "Hello world!"
        
        // Spock's assertions are implicit - any boolean expression is an assertion
        // If any expression in 'then' block is false, the test fails
    }
    
    /**
     * Alternative simple feature method
     * Demonstrates different Spock blocks and styles
     */
    def "says hello to the world"() {
        expect: "Hello world! to be printed"
        // expect: combines given+when+then for simple cases
        def greeting = "Hello world!"
        println greeting
        greeting.contains("world")
    }
    
    /**
     * Parameterized feature method using where: block
     * Demonstrates Spock's data-driven testing capabilities
     */
    def "greets #name with Hello message"() {
        given: "a name"
        def greeting = "Hello $name!"
        
        when: "we print the greeting"
        println greeting
        
        then: "greeting should contain the name"
        greeting.contains(name)
        
        where: "we test with different names"
        name << ["world", "Spock", "Groovy"]
        // The test runs 3 times with different values
    }
}

/*
 * Key Spock Concepts Demonstrated:
 *
 * 1. Specification class - Extends spock.lang.Specification
 * 2. Feature methods - Test methods with descriptive string names
 * 3. Blocks - given:, when:, then:, expect:, where:
 * 4. BDD-style testing - Given-When-Then structure
 * 5. Implicit assertions - Boolean expressions in then: block
 * 6. Data-driven testing - Using where: block with data tables
 * 7. Groovy integration - Can use full Groovy syntax
 *
 * Spock Block Types:
 * - given: (or setup:) - Prepare test fixtures and input data
 * - when: - Perform the action being tested
 * - then: - Verify expected outcomes (implicit assertions)
 * - expect: - Combines when: and then: for simple cases
 * - where: - Provide parameterized test data
 * - cleanup: - Clean up resources after test
 *
 * Running Spock Tests:
 *
 * Method 1: Using Gradle
 *   gradle test                    # Run all tests
 *   gradle test --info             # Run with detailed output
 *   gradle test --tests "*HelloWorld*"  # Run specific test
 *
 * Method 2: Using Gradle wrapper
 *   ./gradlew test
 *   ./gradlew test --info
 *
 * Method 3: From IDE
 *   - IntelliJ IDEA: Right-click → Run 'HelloWorldSpec'
 *   - Eclipse: Right-click → Run As → JUnit Test
 *
 * Expected Output:
 *   Hello world!
 *   Hello world!
 *   Hello world!
 *   Hello Spock!
 *   Hello Groovy!
 *
 * Test Results:
 *   ✓ prints Hello world message
 *   ✓ says hello to the world
 *   ✓ greets world with Hello message
 *   ✓ greets Spock with Hello message
 *   ✓ greets Groovy with Hello message
 *
 * Spock vs Other Testing Frameworks:
 *
 * JUnit (Java):
 *   @Test
 *   public void testHelloWorld() {
 *       String message = "Hello world!";
 *       assertEquals("Hello world!", message);
 *   }
 *
 * RSpec (Ruby):
 *   describe "Hello World" do
 *     it "prints hello world" do
 *       message = "Hello world!"
 *       expect(message).to eq("Hello world!")
 *     end
 *   end
 *
 * Spock (Groovy):
 *   def "prints Hello world message"() {
 *       given: "a greeting message"
 *       def message = "Hello world!"
 *       
 *       expect:
 *       message == "Hello world!"
 *   }
 *
 * Key Advantages of Spock:
 * - Readable, self-documenting tests
 * - Built-in mocking and stubbing
 * - Data-driven testing support
 * - Powerful assertion messages
 * - Groovy's concise syntax
 * - BDD-style structure
 */

