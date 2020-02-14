const sortedSquares = require('./squares-of-a-sorted-array.js');

test('sortedSquares', () => {
  expect(sortedSquares([-4, -1, 0, 3, 10])).toStrictEqual([0, 1, 9, 16, 100]);
  expect(sortedSquares([-7, -3, 2, 3, 11])).toStrictEqual([4, 9, 9, 49, 121]);
  expect(sortedSquares([-1])).toStrictEqual([1]);
  expect(sortedSquares([-4, -1])).toStrictEqual([1, 16]);
});