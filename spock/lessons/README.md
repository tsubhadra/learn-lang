# Spock Lessons - Hello World Example

This directory contains a Spock Hello World example demonstrating basic Spock specification structure.

## What is Spock?

**Spock** is a testing and specification framework for Java and Groovy applications. It combines the best features of:
- **JUnit** - Industry standard testing
- **Mockito** - Mocking framework  
- **RSpec** - BDD-style syntax
- **Groovy** - Expressive, concise language

**Key Features:**
- Clean, readable BDD-style syntax
- Built-in mocking and stubbing
- Data-driven testing
- Powerful assertion messages
- Groovy's expressiveness

## Files

### Source Files
- **`src/test/groovy/com/example/01-HelloWorldSpec.groovy`** - Spock specification
- **`build.gradle`** - Gradle build configuration

### Generated Files (after build)
- **`build/`** - Compiled classes and test results
- **`build/reports/tests/test/index.html`** - HTML test report

## Prerequisites

### Java Installation

**Check if Java is installed:**
```bash
java -version
javac -version
```

**If not installed (using SDKMAN - recommended):**
```bash
# Install SDKMAN
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"

# Install Java
sdk install java 17.0.9-tem
```

### Gradle Installation

**Check if Gradle is installed:**
```bash
gradle --version
```

**If not installed (using SDKMAN - recommended):**
```bash
# Install Gradle
sdk install gradle 8.5

# Verify installation
gradle --version
```

**Alternative (using Homebrew on macOS):**
```bash
brew install gradle
```

**Alternative (manual download):**
```bash
# Download from https://gradle.org/install/
# Extract and add to PATH
export PATH=$PATH:/path/to/gradle/bin
```

## Project Structure

```
spock/lessons/
├── build.gradle                                   # Build configuration
├── src/
│   └── test/
│       └── groovy/
│           └── com/
│               └── example/
│                   └── 01-HelloWorldSpec.groovy  # Spock test
├── build/                                         # Generated (after build)
│   ├── classes/
│   └── reports/
│       └── tests/
│           └── test/
│               └── index.html                     # Test report
└── README.md                                      # This file
```

## How to Run Spock Tests

### Method 1: Using Gradle (Recommended)

```bash
# Navigate to project directory
cd /Users/suthirumavalavan/learn/learn-lang/spock/lessons

# Run all tests
gradle test

# Run with detailed output (shows println statements)
gradle test --info

# Run specific test
gradle test --tests "*HelloWorld*"

# Clean and test
gradle clean test
```

### Method 2: Using Gradle Wrapper

```bash
# If gradlew exists
./gradlew test
./gradlew test --info
```

### Method 3: From IDE

**IntelliJ IDEA:**
1. Open project in IntelliJ
2. Right-click on `01-HelloWorldSpec.groovy`
3. Select "Run 'HelloWorldSpec'"
4. View results in Test Runner window

**Eclipse:**
1. Install Groovy Eclipse plugin
2. Right-click on test file
3. Select "Run As" → "JUnit Test"

### Method 4: Continuous Testing

```bash
# Run tests automatically on file changes
gradle test --continuous
```

## Understanding the Output

### Console Output

```bash
$ gradle test

> Task :test

HelloWorldSpec

Hello world!
Hello world!
Hello world!
Hello Spock!
Hello Groovy!

  ✓ prints Hello world message
  ✓ says hello to the world
  ✓ greets world with Hello message
  ✓ greets Spock with Hello message
  ✓ greets Groovy with Hello message

BUILD SUCCESSFUL in 2s
3 actionable tasks: 3 executed
```

### HTML Test Report

After running tests, open the HTML report:
```bash
open build/reports/tests/test/index.html
```

The report shows:
- Test summary (passed/failed/skipped)
- Execution time
- Test details
- Standard output
- Failure messages (if any)

## Understanding Spock Specifications

### Basic Structure

```groovy
import spock.lang.Specification

class MySpec extends Specification {
    def "feature method name can contain spaces"() {
        given: "setup phase"
        // Prepare test data
        
        when: "action phase"
        // Perform the action
        
        then: "verification phase"
        // Assert expected outcomes
    }
}
```

### Spock Blocks

**1. given: (or setup:) - Setup Phase**
```groovy
given: "a greeting message"
def message = "Hello world!"
```
Prepares test fixtures and input data.

**2. when: - Action Phase**
```groovy
when: "we print the message"
println(message)
```
Performs the action being tested.

**3. then: - Verification Phase**
```groovy
then: "the message should not be empty"
message != null
message.length() > 0
```
Verifies expected outcomes. Every boolean expression is an assertion.

**4. expect: - Combined Phase**
```groovy
expect: "message to equal Hello world!"
message == "Hello world!"
```
Combines when: and then: for simple cases.

**5. where: - Data-Driven Testing**
```groovy
where: "testing with different inputs"
input  | expected
"a"    | 1
"ab"   | 2
"abc"  | 3
```
Provides parameterized test data.

**6. cleanup: - Cleanup Phase**
```groovy
cleanup: "close resources"
connection.close()
```
Cleans up resources after test.

### Feature Methods

```groovy
// Feature method with descriptive name
def "prints Hello world message"() {
    expect:
    println("Hello world!")
    true
}

// Can use spaces and special characters
def "user with age > 18 can vote"() {
    expect:
    def user = new User(age: 20)
    user.canVote()
}
```

### Implicit Assertions

In `then:` and `expect:` blocks, all boolean expressions are assertions:

```groovy
then:
result == 5           // Assertion
result > 0            // Assertion  
result.class == Integer  // Assertion
```

No need for `assert` or `assertEquals`!

### Data-Driven Testing

```groovy
def "maximum of #a and #b is #max"() {
    expect:
    Math.max(a, b) == max
    
    where:
    a | b || max
    1 | 2 || 2
    3 | 1 || 3
    5 | 5 || 5
}
// Runs 3 times with different data
```

## Spock vs Other Testing Frameworks

### JUnit (Java)

```java
@Test
public void testHelloWorld() {
    // Arrange
    String message = "Hello world!";
    
    // Act
    System.out.println(message);
    
    // Assert
    assertEquals("Hello world!", message);
    assertTrue(message.length() > 0);
}
```

### RSpec (Ruby)

```ruby
describe "Hello World" do
  it "prints hello world" do
    message = "Hello world!"
    expect(message).to eq("Hello world!")
    expect(message.length).to be > 0
  end
end
```

### Spock (Groovy)

```groovy
def "prints Hello world message"() {
    given: "a greeting message"
    def message = "Hello world!"
    
    when: "we print the message"
    println(message)
    
    then: "the message should not be empty"
    message == "Hello world!"
    message.length() > 0
}
```

**Key Differences:**
- ✅ More readable with given-when-then structure
- ✅ Method names can contain spaces
- ✅ Implicit assertions (no assertEquals needed)
- ✅ Built-in data-driven testing
- ✅ Better error messages
- ✅ Groovy's concise syntax

## Common Spock Patterns

### 1. Simple Expectation

```groovy
def "list size should be 3"() {
    expect:
    [1, 2, 3].size() == 3
}
```

### 2. Given-When-Then

```groovy
def "adding items to list increases size"() {
    given:
    def list = []
    
    when:
    list << "item"
    
    then:
    list.size() == 1
}
```

### 3. Data Tables

```groovy
def "string #str has length #len"() {
    expect:
    str.length() == len
    
    where:
    str      | len
    "hello"  | 5
    "world"  | 5
    "hi"     | 2
}
```

### 4. Exception Testing

```groovy
def "dividing by zero throws ArithmeticException"() {
    when:
    1 / 0
    
    then:
    thrown(ArithmeticException)
}
```

### 5. Mocking

```groovy
def "calling save on service"() {
    given:
    def mockRepo = Mock(Repository)
    def service = new Service(mockRepo)
    
    when:
    service.save("data")
    
    then:
    1 * mockRepo.save("data")  // Verify called once
}
```

## Building and Running

### Full Build Cycle

```bash
# Clean previous builds
gradle clean

# Compile and run tests
gradle test

# View results
open build/reports/tests/test/index.html

# Generate all reports
gradle build
```

### Gradle Tasks

```bash
# List all tasks
gradle tasks

# Run tests
gradle test

# Clean build directory
gradle clean

# Build project
gradle build

# Run specific test
gradle test --tests "HelloWorldSpec"

# Continuous testing
gradle test --continuous

# Show standard output
gradle test --info

# Debug test execution
gradle test --debug
```

## Configuration Options

### build.gradle Customization

```groovy
test {
    // Show standard output
    testLogging {
        events "passed", "skipped", "failed"
        showStandardStreams = true
    }
    
    // Parallel execution
    maxParallelForks = 4
    
    // JVM options
    jvmArgs '-Xmx512m'
    
    // System properties
    systemProperty 'spock.configuration', 'MyConfig.groovy'
}
```

### Spock Configuration File

Create `src/test/resources/SpockConfig.groovy`:

```groovy
runner {
    // Filter tests by annotation
    include {
        annotation spock.lang.Issue
    }
    
    // Optimization
    optimizeRunOrder true
}
```

## Common Errors and Solutions

### 1. "Could not find spock-core"

**Problem:** Missing dependencies

**Solution:**
```bash
# Check build.gradle has correct dependencies
testImplementation 'org.spockframework:spock-core:2.3-groovy-3.0'

# Update dependencies
gradle clean build --refresh-dependencies
```

### 2. "Class not found"

**Problem:** Wrong package or classpath

**Solution:**
```groovy
// Check package declaration matches directory structure
package com.example  // Must match: src/test/groovy/com/example/
```

### 3. "No tests found"

**Problem:** Test class doesn't extend Specification

**Solution:**
```groovy
import spock.lang.Specification

class MySpec extends Specification {  // Must extend Specification
    def "my test"() {
        expect:
        true
    }
}
```

### 4. "Groovy version mismatch"

**Problem:** Incompatible Groovy version

**Solution:**
```groovy
// In build.gradle, match Groovy version with Spock
testImplementation 'org.spockframework:spock-core:2.3-groovy-3.0'
testImplementation 'org.apache.groovy:groovy:3.0.19'
```

### 5. "Gradle daemon disappeared"

**Solution:**
```bash
# Stop Gradle daemon
gradle --stop

# Run again
gradle test
```

### 6. "Tests not running in IDE"

**Solution:**
```bash
# IntelliJ IDEA:
# 1. File → Invalidate Caches
# 2. Reimport Gradle project
# 3. Ensure Groovy plugin is installed

# Eclipse:
# 1. Install Groovy Eclipse plugin
# 2. Project → Clean
# 3. Refresh project
```

## Advantages of Spock

### 1. Readability
```groovy
// Spock - reads like natural language
def "user with age 20 should be adult"() {
    expect:
    new User(age: 20).isAdult()
}

// vs JUnit
@Test
public void testUserAge20IsAdult() {
    User user = new User();
    user.setAge(20);
    assertTrue(user.isAdult());
}
```

### 2. Powerful Assertions
```groovy
then:
result.size() == 3
// On failure, Spock shows:
// Condition not satisfied:
// result.size() == 3
//        |    |
//        |    2
//        [1, 2]
```

### 3. Built-in Mocking
```groovy
def subscriber = Mock(Subscriber)

when:
publisher.send("hello")

then:
1 * subscriber.receive("hello")
```

### 4. Data-Driven Testing
```groovy
where:
a | b || sum
1 | 2 || 3
3 | 4 || 7
5 | 6 || 11
```

### 5. BDD Support
```groovy
given: "a new user"
when: "the user registers"
then: "the user should be created"
```

## IDE Setup

### IntelliJ IDEA

1. **Install Plugins:**
   - Groovy (usually included)
   - Spock Framework Enhancements (optional)

2. **Import Project:**
   - File → Open → Select `build.gradle`
   - Trust project

3. **Run Tests:**
   - Right-click on spec file
   - Select "Run 'HelloWorldSpec'"

4. **Configure Test Runner:**
   - Settings → Build → Build Tools → Gradle
   - Run tests using: "Gradle" or "IntelliJ IDEA"

### VS Code

1. **Install Extensions:**
   - Groovy Language Server
   - Gradle for Java
   - Test Runner for Java

2. **Open Project:**
   - Open folder containing `build.gradle`

3. **Run Tests:**
   - Open Test Explorer
   - Click run button next to test

### Eclipse

1. **Install Plugins:**
   - Groovy Development Tools
   - Buildship Gradle Integration

2. **Import Project:**
   - File → Import → Gradle → Existing Gradle Project

3. **Run Tests:**
   - Right-click on spec
   - Run As → JUnit Test

## Next Steps

After mastering Hello World, explore:

1. **Assertions and Conditions**
   - Implicit assertions
   - Exception testing
   - Custom matchers

2. **Data-Driven Testing**
   - Where blocks
   - Data tables
   - Data pipes

3. **Mocking and Stubbing**
   - Mock objects
   - Stubs
   - Spies
   - Interaction testing

4. **Advanced Features**
   - @Shared fields
   - @Unroll annotation
   - Specifications lifecycle
   - Extension points

5. **Integration Testing**
   - Spring Boot integration
   - Database testing
   - REST API testing

6. **Best Practices**
   - Test organization
   - Naming conventions
   - DRY principles
   - Page objects (for UI testing)

## Resources

- **Spock Official Docs**: https://spockframework.org/
- **Spock GitHub**: https://github.com/spockframework/spock
- **Groovy Documentation**: https://groovy-lang.org/documentation.html
- **Gradle User Guide**: https://docs.gradle.org/
- **Spock Primer**: http://spockframework.org/spock/docs/2.3/
- **Project Setup Guide**: See `../docs/initial-setup.txt`

## Tips

1. **Use descriptive test names**: `def "user can login with valid credentials"()`
2. **Keep tests focused**: One concept per test
3. **Use given-when-then**: Makes tests more readable
4. **Leverage data tables**: For parameterized testing
5. **Use Groovy power features**: Closures, GString, etc.
6. **Mock external dependencies**: Keep tests fast and isolated
7. **Use @Unroll**: For better reporting of data-driven tests
8. **Write specifications, not tests**: Think behavior, not implementation
9. **Use interaction testing**: Verify how objects collaborate
10. **Keep it simple**: Spock's power is in its simplicity

## Troubleshooting

### Tests pass but no output visible

```bash
# Run with --info to see println output
gradle test --info

# Or configure in build.gradle
test {
    testLogging.showStandardStreams = true
}
```

### Gradle version issues

```bash
# Use wrapper with specific version
gradle wrapper --gradle-version 8.5
./gradlew test
```

### JAVA_HOME not set

```bash
# Set JAVA_HOME
export JAVA_HOME=$(/usr/libexec/java_home)

# Or for specific version
export JAVA_HOME=$(/usr/libexec/java_home -v 17)
```

## Summary

- ✅ Spock is a BDD testing framework for JVM
- ✅ Uses Groovy's expressive syntax
- ✅ Given-When-Then structure
- ✅ Implicit assertions
- ✅ Built-in mocking and data-driven testing
- ✅ Excellent error messages
- ✅ More readable than JUnit
- ✅ Great for both unit and integration tests

Happy Spock testing! 🖖

