const kWeakestRows = require('./the-k-weakest-rows-in-a-matrix.js');

test('kWeakestRows', () => {
  expect(kWeakestRows(
    [[1, 1, 0, 0, 0],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 1, 1, 1, 1]], 3)).toStrictEqual([2, 0, 3]);

  expect(kWeakestRows([
    [1, 0],
    [1, 0],
    [1, 0],
    [1, 1]], 4
  )).toStrictEqual([0, 1, 2, 3]);

  expect(kWeakestRows(
    [[1, 0, 0, 0],
    [1, 1, 1, 1],
    [1, 0, 0, 0],
    [1, 0, 0, 0]], 2)).toStrictEqual([0, 2]);

  expect(kWeakestRows(
    [[1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1]], 1)).toStrictEqual([0]);

  expect(kWeakestRows(
    [[1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1]], 2)).toStrictEqual([0, 1]);


  expect(kWeakestRows(
    [[1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1],
    [1, 1, 1, 0, 1, 1],
    [1, 1, 1, 1, 1, 1]], 3)).toStrictEqual([2, 0, 1]);

});