const maximum69Number = require('./maximum-69-number')

test('maximum69Number', () => {
  expect(maximum69Number(9669)).toBe(9969)
  expect(maximum69Number(9996)).toBe(9999)
  expect(maximum69Number(9999)).toBe(9999)
})