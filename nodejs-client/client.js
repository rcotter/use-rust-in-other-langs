const lib = require('./lib');

async function run() {
  const { add } = await lib(); 
  const sum = add(5, 6);
    console.log(`5 + 6 = ${sum}`);
}

run();