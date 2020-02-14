const decompressRLElist = require('./decompress-run-length-encoded-list.js');

test('decompressRLElist', () => {
  expect(decompressRLElist([1, 2, 3, 4])).toStrictEqual([2, 4, 4, 4]);
});
