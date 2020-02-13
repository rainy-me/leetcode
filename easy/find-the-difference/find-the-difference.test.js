const findTheDifference = require('./find-the-difference.js');

test('findTheDifference', () => {
  expect(findTheDifference("abcd", "abcde")).toBe("e");
});