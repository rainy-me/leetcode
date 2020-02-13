const sortArrayByParityII = require('./sort-array-by-parity-ii.js');

test('sortArrayByParityII', () => {
  expect(sortArrayByParityII([4, 2, 5, 7]).every((k, v) => (k & 1) === (v & 1))).toBe(true);
});