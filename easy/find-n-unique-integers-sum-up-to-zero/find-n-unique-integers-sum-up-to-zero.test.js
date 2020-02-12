const sumZero = require('./find-n-unique-integers-sum-up-to-zero.js');

test('sumZero', () => {
  const r100 = sumZero(100)
  const r5 = sumZero(5)
  const r4 = sumZero(4)
  const r3 = sumZero(3)
  const r2 = sumZero(2)
  const r1 = sumZero(1)

  const sum = arr => arr.reduce((a, b) => a + b)

  expect(r100.length).toBe(100);
  expect(r5.length).toBe(5);
  expect(r4.length).toBe(4);
  expect(r3.length).toBe(3);
  expect(r2.length).toBe(2);
  expect(r1.length).toBe(1);

  expect(sum(r100)).toBe(0);
  expect(sum(r5)).toBe(0);
  expect(sum(r4)).toBe(0);
  expect(sum(r3)).toBe(0);
  expect(sum(r2)).toBe(0);
  expect(sum(r1)).toBe(0);
});