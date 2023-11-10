const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

rl.question('Enter a number: ', (input) => {
  function calculateSumOfSquares(values) {
    return values.reduce((sum, x) => sum + x * x, 0);
  }

  // Process the input
  const number = parseInt(input, 10);
  const testData = Array.from({ length: number }, (_, i) => i + 1);

  // MAIN PROCESS
  console.time('node');

  const nodeCalc = calculateSumOfSquares(testData);

  console.timeEnd('node');
  console.log(`Node Factorial of ${number} is: ${nodeCalc}`);

  rl.close();
});
