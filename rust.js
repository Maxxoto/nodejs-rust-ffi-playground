const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

rl.question('Enter a number: ', (input) => {
  const addon = require('.');

  // Process the input
  const number = parseInt(input, 10);
  const testData = Array.from({ length: number }, (_, i) => i + 1);

  // Use the number in your logic (e.g., calculate factorial)
  console.time('rust');
  const rustCalc = addon.calculateSumOfSquare(testData);
  console.timeEnd('rust');
  console.log(`Rust result of ${number} is: ${rustCalc}`);

  rl.close();
});
