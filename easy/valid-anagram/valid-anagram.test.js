const isAnagram = require('./valid-anagram.js');

test('isAnagram', () => {
  expect(isAnagram("anagram", "nagaram")).toBe(true);
  expect(isAnagram("rat", "car")).toBe(false);
  expect(isAnagram("aa", "bb")).toBe(false);
  expect(isAnagram("a", "ab")).toBe(false);
  expect(isAnagram("", "")).toBe(true);
  expect(isAnagram("ac", "bb")).toBe(false);
  expect(isAnagram("nl", "cx")).toBe(false);
  expect(isAnagram("aacc", "ccac")).toBe(false);
});