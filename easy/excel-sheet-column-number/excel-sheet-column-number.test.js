const titleToNumber = require('./excel-sheet-column-number.js');

test('titleToNumber', () => {
  expect(titleToNumber("A")).toBe(1);
  expect(titleToNumber("AB")).toBe(28);
  expect(titleToNumber("ZY")).toBe(701);
});