const selfDividingNumbers = require('./self-dividing-numbers.js');

test('selfDividingNumbers', () => {
  expect(selfDividingNumbers(1, 22)).toStrictEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]);
});