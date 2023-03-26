const lib = require('../lib');
const loadTestFile = require('./support/loadTestFile');

let add = null;

describe('tests', () => {
    beforeAll(async () => {
        add = (await lib()).add;
    });

    test('adds two numbers', () => {
        const lines = loadTestFile('add_inputs.txt');

        for (let i = 0; i < lines.length; i++) {
            let [a, b, expectedSum, nodeJsExpectedSum] = lines[i].split(',').map(s => parseFloat(s))
            expectedSum = nodeJsExpectedSum == undefined ? expectedSum : nodeJsExpectedSum;    
            expect(add(a, b)).toEqual(expectedSum);
        }
    });
});
