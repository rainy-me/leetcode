const allCellsDistOrder = require('./matrix-cells-in-distance-order.js');

test('allCellsDistOrder', () => {
  expect(allCellsDistOrder(1, 2, 0, 0)).toStrictEqual([[0, 0], [0, 1]]);
  expect(allCellsDistOrder(2, 2, 0, 1)).toStrictEqual([[0, 1], [0, 0], [1, 1], [1, 0]]);
  expect(allCellsDistOrder(2, 3, 1, 2)).toStrictEqual([[1, 2], [0, 2], [1, 1], [0, 1], [1, 0], [0, 0]]);
});