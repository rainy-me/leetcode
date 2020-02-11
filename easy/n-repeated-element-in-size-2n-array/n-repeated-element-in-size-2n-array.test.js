const repeatedNTimes = require('./n-repeated-element-in-size-2n-array')

test('repeatedNTimes', () => {
  expect(repeatedNTimes([1, 1, 2, 3])).toBe(1);
  expect(repeatedNTimes([2, 1, 2, 5, 3, 2])).toBe(2);
  expect(repeatedNTimes([5, 1, 5, 2, 5, 3, 5, 4])).toBe(5);
})