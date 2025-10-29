const readline = require('readline');

// Create interface for reading input from terminal
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

// Ask question and get input
rl.question('What is your name? ', (name) => {
    console.log(`Hello ${name}, how are you?`);
    rl.close();
});

