const hasAlternatingBits = require('./binary-number-with-alternating-bits.js');

test('hasAlternatingBits', () => {
  expect(hasAlternatingBits(5)).toBe(true);
  expect(hasAlternatingBits(7)).toBe(false);
  expect(hasAlternatingBits(11)).toBe(false);
  expect(hasAlternatingBits(10)).toBe(true);
});