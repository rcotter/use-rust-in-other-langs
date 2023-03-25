const fs = require('fs');
const path = require('path');

// Get array of file lines/rows. Each line is unparsed.
module.exports = function(fileName) {
    const testDataPath = path.join(__dirname, '..', '..', '..', 'test-input', fileName);
    const fileContent = fs.readFileSync(testDataPath, 'utf8');
    return fileContent.trim().split('\n').slice(1); // Skips header line
}