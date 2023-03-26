const lib = require('./lib');

async function run() {
  const { add } = await lib(); 
  const sum = add(5.1, 6.3);
    console.log(`5.1 + 6.3 = ${sum}`);
}

run();