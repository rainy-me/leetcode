const moveZeroes = require('./move-zeroes.js');

test('moveZeroes', () => {
  n = [0, 1, 0, 3, 12]
  moveZeroes(n)
  expect(n).toStrictEqual([1, 3, 12, 0, 0]);

  n = [0, 0, 1]
  moveZeroes(n)
  expect(n).toStrictEqual([1, 0, 0]);
});