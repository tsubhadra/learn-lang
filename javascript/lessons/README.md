# JavaScript Lessons - Hello World Examples

This directory contains JavaScript Hello World examples demonstrating how to run JavaScript in different environments.

## Files

### JavaScript Files
- **`01-hello-world.js`** - Node.js console application (2.2KB)
- **`01-hello-world.html`** - Browser-based example (2.5KB)

## Prerequisites

### Node.js and npm

JavaScript can run in browsers (no installation needed) or with Node.js for server-side execution.

**Check if Node.js is installed:**
```bash
node --version
npm --version
```

**If not installed, install Node.js:**

**Using nvm (Recommended - Node Version Manager):**
```bash
# Install nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash

# Restart terminal or source profile
source ~/.zshrc  # or ~/.bashrc

# Install Node.js
nvm install --lts

# Verify
node --version
npm --version
```

**Using Homebrew (macOS):**
```bash
brew install node
```

**Using Package Manager (Linux):**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install nodejs npm

# Or with NodeSource
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt-get install -y nodejs
```

**Manual Download:**
- Visit https://nodejs.org/
- Download LTS version
- Run installer

## How to Run JavaScript

### Method 1: Node.js (Command Line)

**Run the JavaScript file:**
```bash
node 01-hello-world.js
```

**Output:**
```
Hello world!
```

**Alternative: Make executable (Unix/Linux/macOS):**
```bash
# Add shebang to first line: #!/usr/bin/env node
chmod +x 01-hello-world.js
./01-hello-world.js
```

### Method 2: Browser

**Option A: Open HTML file directly**
1. Double-click `01-hello-world.html`
2. Opens in your default browser
3. See "Hello world!" on the page
4. Press F12 to open Developer Tools
5. Click "Console" tab to see console.log() output

**Option B: Use Live Server (VSCode)**
1. Install "Live Server" extension in VSCode
2. Right-click on `01-hello-world.html`
3. Select "Open with Live Server"
4. Browser opens automatically with live reload

**Option C: Python HTTP Server**
```bash
# Python 3
python3 -m http.server 8000

# Python 2
python -m SimpleHTTPServer 8000

# Then open: http://localhost:8000/01-hello-world.html
```

**Option D: Node.js HTTP Server**
```bash
# Install http-server globally
npm install -g http-server

# Start server
http-server

# Open: http://localhost:8080/01-hello-world.html
```

### Method 3: Browser Console (Interactive)

1. Open any browser (Chrome, Firefox, Edge, Safari)
2. Press F12 or right-click → Inspect
3. Go to Console tab
4. Type JavaScript directly:
   ```javascript
   console.log("Hello world!");
   ```
5. Press Enter

### Method 4: Online JavaScript Editors

**No installation needed:**
- **JSFiddle**: https://jsfiddle.net/
- **CodePen**: https://codepen.io/
- **JS Bin**: https://jsbin.com/
- **Repl.it**: https://replit.com/

## Understanding the Files

### File Structure

```
lessons/
├── 01-hello-world.js       # Node.js version
├── 01-hello-world.html     # Browser version
└── README.md               # This file
```

### JavaScript vs Other Languages

**Java (requires compilation):**
```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}
// Compile: javac HelloWorld.java
// Run: java HelloWorld
```

**JavaScript (interpreted, no compilation):**
```javascript
console.log("Hello world!");
// Run: node hello.js
```

**Key Differences:**
- ❌ No compilation step required
- ❌ No class definition needed
- ❌ No main() function required
- ✅ Direct execution
- ✅ Simpler syntax
- ✅ Runs in browsers without any setup

## JavaScript Execution Environments

### 1. Node.js (Server-Side)

**Characteristics:**
- Runs on V8 JavaScript engine (same as Chrome)
- Access to file system
- Can create servers
- Package management with npm
- Used for backend development

**Example:**
```javascript
// Node.js specific code
const fs = require('fs');
console.log("Hello world!");
```

**Run:**
```bash
node 01-hello-world.js
```

### 2. Browser (Client-Side)

**Characteristics:**
- Runs in web browsers
- Access to DOM (Document Object Model)
- Can manipulate web pages
- Limited file system access (security)
- Used for frontend development

**Example:**
```javascript
// Browser specific code
document.body.innerHTML = "Hello world!";
console.log("Hello world!");
```

**Run:**
- Open HTML file in browser
- Check console (F12 → Console)

### 3. Comparison

| Feature | Node.js | Browser |
|---------|---------|---------|
| Environment | Server | Client |
| File Access | ✅ Full | ❌ Limited |
| DOM Access | ❌ No | ✅ Yes |
| Modules | CommonJS/ES6 | ES6 |
| Use Case | Backend APIs | Interactive UI |

## Program Structure Explained

### Basic JavaScript Structure

```javascript
// 1. No imports needed for basic programs
// 2. No class definition required
// 3. No main() function needed

// Code executes from top to bottom
console.log("Hello world!");

// Variables (optional)
const message = "Hello world!";
console.log(message);

// Functions (optional)
function greet() {
    console.log("Hello world!");
}
greet();
```

### Modern JavaScript (ES6+)

```javascript
// Template literals
const name = "world";
console.log(`Hello ${name}!`);

// Arrow functions
const greet = () => console.log("Hello world!");
greet();

// Destructuring
const { log } = console;
log("Hello world!");

// Modules (ES6)
export const greeting = "Hello world!";
```

## Console Methods

### Different Ways to Print

```javascript
// 1. console.log() - Standard output
console.log("Hello world!");

// 2. console.info() - Informational (same as log)
console.info("Hello world!");

// 3. console.warn() - Warning (yellow in console)
console.warn("Hello world!");

// 4. console.error() - Error (red in console)
console.error("Hello world!");

// 5. console.debug() - Debug message
console.debug("Hello world!");

// 6. console.table() - Display as table (arrays/objects)
console.table([{msg: "Hello world!"}]);

// 7. process.stdout.write() - Node.js only (no newline)
process.stdout.write("Hello world!\n");
```

### String Concatenation Methods

```javascript
// 1. Plus operator
console.log("Hello" + " " + "world!");

// 2. Template literals (ES6+) - Recommended
const name = "world";
console.log(`Hello ${name}!`);

// 3. concat() method
console.log("Hello".concat(" ", "world!"));

// 4. Array join()
console.log(["Hello", "world!"].join(" "));

// 5. Multiple arguments
console.log("Hello", "world!");
```

## Running with npm Scripts

Create `package.json`:
```json
{
  "name": "hello-world",
  "version": "1.0.0",
  "scripts": {
    "start": "node 01-hello-world.js",
    "dev": "nodemon 01-hello-world.js"
  }
}
```

**Run:**
```bash
npm start
```

## Common JavaScript Patterns

### 1. Immediately Invoked Function Expression (IIFE)

```javascript
(function() {
    console.log("Hello world!");
})();
```

### 2. Async/Await (Modern)

```javascript
async function greet() {
    await Promise.resolve();
    console.log("Hello world!");
}
greet();
```

### 3. Modular Approach

```javascript
// hello.js
function greet() {
    return "Hello world!";
}

module.exports = greet;

// main.js
const greet = require('./hello');
console.log(greet());
```

### 4. ES6 Modules

```javascript
// hello.js
export function greet() {
    return "Hello world!";
}

// main.js
import { greet } from './hello.js';
console.log(greet());
```

## Debugging JavaScript

### In Node.js

```bash
# Run with debugger
node --inspect 01-hello-world.js

# Run with automatic break
node --inspect-brk 01-hello-world.js

# Open Chrome and go to: chrome://inspect
```

### In Browser

```javascript
// Add breakpoint in code
debugger;
console.log("Hello world!");

// Or use browser DevTools:
// F12 → Sources → Set breakpoint
```

### Console Debugging

```javascript
console.log("Value:", someVariable);
console.log("Type:", typeof someVariable);
console.dir(someObject);  // Show object properties
console.trace();          // Show stack trace
```

## Common Errors and Solutions

### 1. "node: command not found"

**Solution:**
```bash
# Install Node.js
nvm install --lts

# Or check PATH
echo $PATH

# Add Node to PATH if installed
export PATH="/usr/local/bin:$PATH"
```

### 2. "SyntaxError: Unexpected token"

**Problem:** Code syntax error

**Solution:**
```javascript
// Wrong
console.log("Hello world!)  // Missing closing quote

// Correct
console.log("Hello world!");
```

### 3. "ReferenceError: [variable] is not defined"

**Problem:** Using undefined variable

**Solution:**
```javascript
// Wrong
console.log(message);  // message not defined

// Correct
const message = "Hello world!";
console.log(message);
```

### 4. Module import/export errors

**Solution:**
- Use CommonJS for Node.js: `require()` / `module.exports`
- Use ES6 modules: `import` / `export` (add `"type": "module"` to package.json)

## JavaScript Features Demonstrated

### ES5 (Legacy)
```javascript
var message = "Hello world!";
console.log(message);
```

### ES6+ (Modern)
```javascript
// const/let instead of var
const message = "Hello world!";

// Template literals
console.log(`Message: ${message}`);

// Arrow functions
const greet = () => console.log("Hello world!");

// Destructuring
const { log } = console;
log("Hello world!");
```

## File Sizes

```bash
$ ls -lh
-rw-r--r--  2.2K  01-hello-world.js      # JavaScript source
-rw-r--r--  2.5K  01-hello-world.html    # HTML with JavaScript
```

**Key Points:**
- No compilation = No separate executable file
- Source code runs directly
- Same file for development and production (usually)
- Can be minified for production (reduces size)

## Next Steps

After mastering Hello World, explore:

1. **Variables and Data Types**
   - `let`, `const`, `var`
   - Numbers, strings, booleans, objects, arrays

2. **Functions**
   - Function declarations
   - Arrow functions
   - Callbacks

3. **Control Structures**
   - if/else statements
   - for, while loops
   - switch statements

4. **DOM Manipulation** (Browser)
   - `document.getElementById()`
   - `addEventListener()`
   - Dynamic HTML

5. **Async Programming**
   - Promises
   - async/await
   - Fetch API

6. **Modules**
   - CommonJS (`require`)
   - ES6 (`import/export`)

7. **Node.js Basics**
   - File system
   - HTTP servers
   - Express.js

## Useful Commands

```bash
# Check Node.js version
node --version
node -v

# Check npm version
npm --version
npm -v

# Run JavaScript file
node filename.js

# Run with environment variables
NODE_ENV=production node filename.js

# Run with debugger
node --inspect filename.js

# Evaluate JavaScript expression
node -e "console.log('Hello world!')"

# Open Node.js REPL (interactive)
node

# Install package globally
npm install -g package-name

# Initialize new project
npm init
npm init -y  # Skip questions

# Install dependencies
npm install

# Run npm script
npm start
npm run dev
```

## Resources

- **MDN Web Docs**: https://developer.mozilla.org/en-US/docs/Web/JavaScript
- **JavaScript.info**: https://javascript.info/
- **Node.js Docs**: https://nodejs.org/docs/
- **Project Setup Guide**: See `../docs/initial-setup.txt`

## Tips

1. **Use Modern JavaScript (ES6+)**: Template literals, arrow functions, const/let
2. **Semicolons**: Optional but recommended for clarity
3. **Console.log()**: Your best friend for debugging
4. **Browser DevTools**: Essential for frontend development (F12)
5. **Node.js**: Essential for backend development
6. **npm**: Package manager - learn it early
7. **Async**: JavaScript is single-threaded but async-capable

## Troubleshooting

### Script doesn't run in browser

1. Check browser console for errors (F12)
2. Ensure script tag is correct: `<script src="01-hello-world.js"></script>`
3. Check file path is correct
4. Make sure JavaScript is enabled in browser

### Node.js script errors

```bash
# Clear npm cache if issues
npm cache clean --force

# Reinstall Node.js
nvm install --lts
nvm use --lts
```

## Summary

- ✅ JavaScript is interpreted (no compilation needed)
- ✅ Runs in browsers and Node.js
- ✅ `console.log()` for output
- ✅ No main() function required
- ✅ Simple and flexible syntax
- ✅ Two files: .js for Node.js, .html for browser
- ✅ Modern JavaScript (ES6+) recommended

Happy coding! 🎉

