const shortestToChar = require('./shortest-distance-to-a-character.js');

test('shortestToChar', () => {
  expect(shortestToChar("loveleetcode", "e")).toStrictEqual([3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
  expect(shortestToChar("eloveleetcode", "e")).toStrictEqual([0, 1, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]);
  expect(shortestToChar("aaba", "b")).toStrictEqual([2, 1, 0, 1]);
  expect(shortestToChar("aabaa", "b")).toStrictEqual([2, 1, 0, 1, 2]);
});