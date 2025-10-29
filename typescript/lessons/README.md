# TypeScript Lessons - Hello World Example

This directory contains a TypeScript Hello World example demonstrating basic TypeScript features and compilation.

## Files

### Source Files
- **`01-hello-world.ts`** - TypeScript source code (4.0KB)
- **`01-hello-world.js`** - Compiled JavaScript output (2.4KB)
- **`package.json`** - Node.js project configuration
- **`package-lock.json`** - Dependency lock file

### Directories
- **`node_modules/`** - Installed dependencies (TypeScript compiler)

## What is TypeScript?

TypeScript is a **typed superset of JavaScript** that compiles to plain JavaScript. It adds:
- **Static typing** - Catch errors at compile-time
- **Interfaces** - Define object shapes
- **Enums** - Named constants
- **Generics** - Type-safe reusable code
- **Access modifiers** - public, private, protected
- **Advanced IDE support** - Better autocomplete and refactoring

## Prerequisites

### Node.js and npm

TypeScript requires Node.js and npm.

**Check if installed:**
```bash
node --version
npm --version
```

**If not installed, see:**
- JavaScript setup guide in `../../javascript/docs/initial-setup.txt`
- Or install via nvm, Homebrew, or nodejs.org

### TypeScript

**Install TypeScript globally (optional):**
```bash
npm install -g typescript

# Verify
tsc --version
```

**Or use project-level TypeScript (recommended):**
```bash
# Already installed in this directory
npm install --save-dev typescript
```

## How to Compile and Run

### Method 1: Using tsc + node (Standard)

```bash
# Step 1: Compile TypeScript to JavaScript
npx tsc 01-hello-world.ts

# Step 2: Run the compiled JavaScript
node 01-hello-world.js
```

**Output:**
```
Hello world!
```

### Method 2: One-Line Compile and Run

```bash
# Compile and run in one command
npx tsc 01-hello-world.ts && node 01-hello-world.js
```

### Method 3: Using ts-node (Direct Execution)

```bash
# Install ts-node
npm install --save-dev ts-node

# Run TypeScript directly without compilation
npx ts-node 01-hello-world.ts
```

### Method 4: Using tsx (Fastest)

```bash
# Install tsx
npm install --save-dev tsx

# Run TypeScript directly (faster than ts-node)
npx tsx 01-hello-world.ts
```

### Method 5: Using npm Scripts

**Add to package.json:**
```json
{
  "scripts": {
    "build": "tsc 01-hello-world.ts",
    "start": "node 01-hello-world.js",
    "dev": "ts-node 01-hello-world.ts",
    "dev:fast": "tsx 01-hello-world.ts"
  }
}
```

**Run:**
```bash
npm run build   # Compile
npm start       # Run compiled JS
npm run dev     # Run with ts-node
npm run dev:fast # Run with tsx
```

## Understanding the Files

### File Size Comparison

```bash
$ ls -lh
-rw-r--r--  4.0K  01-hello-world.ts    # TypeScript source
-rw-r--r--  2.4K  01-hello-world.js    # Compiled JavaScript
```

**Key Points:**
- `.ts` files are TypeScript source (human-readable with types)
- `.js` files are compiled JavaScript (types removed)
- JavaScript is smaller because type annotations are stripped
- Both files are human-readable text

### Compilation Process

```
TypeScript (.ts)
        ↓
      [tsc]  ← TypeScript Compiler
        ↓
   JavaScript (.js)
        ↓
     [Node.js]
        ↓
    Execution
```

**What happens during compilation:**
1. **Type Checking** - Validates all types are correct
2. **Type Erasure** - Removes all type annotations
3. **Transpilation** - Converts to JavaScript (ES5/ES6/etc.)
4. **Output** - Creates `.js` file

## TypeScript vs JavaScript

### JavaScript (Dynamic Typing)

```javascript
// No type checking
function add(a, b) {
    return a + b;
}

add(5, 3);      // 8
add("5", "3");  // "53" (string concatenation - might be unexpected!)
```

### TypeScript (Static Typing)

```typescript
// Type checking at compile-time
function add(a: number, b: number): number {
    return a + b;
}

add(5, 3);      // 8 ✓
add("5", "3");  // ❌ Compile error: Argument of type 'string' is not assignable to parameter of type 'number'
```

### Key Differences

| Feature | JavaScript | TypeScript |
|---------|-----------|------------|
| **Typing** | Dynamic (runtime) | Static (compile-time) |
| **Compilation** | ❌ No | ✅ Yes |
| **Interfaces** | ❌ No | ✅ Yes |
| **Enums** | ❌ No | ✅ Yes |
| **Generics** | ❌ No | ✅ Yes |
| **Error Detection** | Runtime | Compile-time |
| **IDE Support** | Good | Excellent |
| **Learning Curve** | Easy | Medium |
| **File Extension** | .js | .ts |

## TypeScript Features Demonstrated

### 1. Type Annotations

```typescript
const message: string = "Hello world!";
const count: number = 42;
const isActive: boolean = true;
```

### 2. Function Type Annotations

```typescript
function greet(name: string): string {
    return `Hello ${name}!`;
}
```

### 3. Interfaces

```typescript
interface User {
    name: string;
    age: number;
}

const user: User = {
    name: "Alice",
    age: 30
};
```

### 4. Type Aliases

```typescript
type ID = string | number;
type Status = "active" | "inactive";
```

### 5. Classes with Access Modifiers

```typescript
class Person {
    private name: string;
    public age: number;
    
    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }
}
```

### 6. Enums

```typescript
enum Direction {
    Up,
    Down,
    Left,
    Right
}
```

### 7. Generics

```typescript
function identity<T>(value: T): T {
    return value;
}
```

### 8. Union Types

```typescript
type StringOrNumber = string | number;
```

### 9. Optional Properties

```typescript
interface Config {
    apiKey: string;
    timeout?: number;  // Optional
}
```

### 10. Readonly Properties

```typescript
interface Point {
    readonly x: number;
    readonly y: number;
}
```

## Configuration

### Creating tsconfig.json

```bash
# Generate default configuration
npx tsc --init
```

### Sample tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "outDir": "./dist",
    "rootDir": "./",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["*.ts"],
  "exclude": ["node_modules"]
}
```

**Then compile all files:**
```bash
npx tsc
```

## Common Commands

```bash
# Compile single file
npx tsc filename.ts

# Compile all files (uses tsconfig.json)
npx tsc

# Watch mode (auto-recompile on changes)
npx tsc --watch
npx tsc -w

# Compile with specific target
npx tsc --target ES2020 filename.ts

# Check for errors without emitting files
npx tsc --noEmit

# Show compiler version
npx tsc --version

# Initialize tsconfig.json
npx tsc --init

# Run TypeScript directly (no compilation needed)
npx ts-node filename.ts
npx tsx filename.ts
```

## Common Errors and Solutions

### 1. "tsc: command not found"

**Solution:**
```bash
# Install TypeScript
npm install --save-dev typescript

# Or globally
npm install -g typescript

# Use npx to run without global install
npx tsc filename.ts
```

### 2. "Cannot find name 'require'" or "Cannot find name 'module'"

**Solution:**
```bash
# Install Node.js type definitions
npm install --save-dev @types/node
```

### 3. Type errors in compilation

**Example Error:**
```
error TS2345: Argument of type 'string' is not assignable to parameter of type 'number'
```

**Solution:**
- Fix the types to match
- Use type assertions if you're certain: `value as number`
- Use `any` type (not recommended): `value: any`

### 4. "Module not found"

**Solution:**
```json
// Add to tsconfig.json
{
  "compilerOptions": {
    "moduleResolution": "node",
    "esModuleInterop": true
  }
}
```

## TypeScript vs Compiled Languages

### Java (Statically Typed, Compiled to Bytecode)

```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello world!");
    }
}
// javac → .class → java
```

### TypeScript (Statically Typed, Compiled to JavaScript)

```typescript
console.log("Hello world!");
// tsc → .js → node
```

### JavaScript (Dynamically Typed, Interpreted)

```javascript
console.log("Hello world!");
// node (no compilation)
```

## Why Use TypeScript?

### Advantages

✅ **Catch errors early** - Find bugs at compile-time, not runtime
✅ **Better IDE support** - Autocomplete, refactoring, go-to-definition
✅ **Self-documenting code** - Types serve as inline documentation
✅ **Refactoring confidence** - Change code safely with type checking
✅ **Team collaboration** - Clear contracts between code components
✅ **Gradual adoption** - Add types incrementally to existing JS
✅ **Modern JavaScript** - Use latest features with backward compatibility

### When to Use TypeScript

- ✅ Large codebases
- ✅ Team projects
- ✅ Long-term maintenance
- ✅ Complex applications
- ✅ API development
- ✅ Libraries and frameworks

### When JavaScript Might Be Sufficient

- Simple scripts
- Small projects
- Rapid prototyping
- Learning programming basics

## Project Structure Examples

### Simple TypeScript Project

```
project/
├── src/
│   └── index.ts
├── dist/
│   └── index.js
├── node_modules/
├── package.json
├── tsconfig.json
└── README.md
```

### TypeScript with Tests

```
project/
├── src/
│   ├── index.ts
│   └── utils.ts
├── tests/
│   ├── index.test.ts
│   └── utils.test.ts
├── dist/
├── node_modules/
├── package.json
├── tsconfig.json
└── jest.config.js
```

## Debugging TypeScript

### In VSCode

Create `.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "node",
      "request": "launch",
      "name": "Debug TypeScript",
      "runtimeArgs": ["-r", "ts-node/register"],
      "args": ["${workspaceFolder}/01-hello-world.ts"]
    }
  ]
}
```

Press F5 to debug.

### With Source Maps

```json
{
  "compilerOptions": {
    "sourceMap": true
  }
}
```

Then debug the JS file with source maps pointing back to TS.

## Next Steps

After mastering Hello World, explore:

1. **TypeScript Basics**
   - Primitive types (string, number, boolean)
   - Arrays and tuples
   - Objects and interfaces
   - Union and intersection types

2. **Functions**
   - Function types
   - Optional and default parameters
   - Rest parameters
   - Overloading

3. **Advanced Types**
   - Generics
   - Utility types (Partial, Required, Pick, Omit)
   - Conditional types
   - Mapped types

4. **Classes and OOP**
   - Class inheritance
   - Abstract classes
   - Access modifiers
   - Static members

5. **Modules**
   - Import/export
   - Namespaces
   - Module resolution

6. **Type Guards**
   - typeof, instanceof
   - Custom type guards
   - Discriminated unions

7. **Decorators** (experimental)
   - Class decorators
   - Method decorators
   - Property decorators

## Resources

- **Official TypeScript Docs**: https://www.typescriptlang.org/docs/
- **TypeScript Playground**: https://www.typescriptlang.org/play
- **TypeScript Deep Dive**: https://basarat.gitbook.io/typescript/
- **Type Challenges**: https://github.com/type-challenges/type-challenges
- **Project Setup Guide**: See `../docs/initial-setup.txt`

## Tips

1. **Start strict**: Use `"strict": true` in tsconfig.json
2. **Avoid `any`**: Use `unknown` if you must, or specific types
3. **Use interfaces**: For object shapes and contracts
4. **Type inference**: Let TypeScript infer when obvious
5. **Utility types**: Learn Partial, Pick, Omit, Required
6. **Read compiler errors**: They're usually very helpful
7. **Use ESLint**: Add TypeScript ESLint rules
8. **Enable editor features**: Let TypeScript enhance your IDE

## Troubleshooting

### Compilation is slow

**Solution:**
```json
{
  "compilerOptions": {
    "incremental": true,
    "skipLibCheck": true
  }
}
```

### Types don't match imported library

**Solution:**
```bash
# Install type definitions
npm install --save-dev @types/library-name

# For example:
npm install --save-dev @types/node
npm install --save-dev @types/express
```

### "Cannot redeclare block-scoped variable"

**Solution:**
Add `export {}` at the end of the file to make it a module.

## Summary

- ✅ TypeScript adds **static typing** to JavaScript
- ✅ **Compiles to JavaScript** - runs anywhere JS runs
- ✅ **Catches errors at compile-time** - safer code
- ✅ **Better tooling** - enhanced IDE features
- ✅ **Gradual adoption** - add types incrementally
- ✅ Two files created: `.ts` (source) and `.js` (output)
- ✅ Three execution methods: tsc + node, ts-node, tsx

Happy coding with TypeScript! 🚀

