const lengthOfLongestSubstring = require('./longest-substring-without-repeating-characters.js');

test('lengthOfLongestSubstring', () => {
  expect(lengthOfLongestSubstring("abcabcbb")).toBe(3);
  expect(lengthOfLongestSubstring("bbbbb")).toBe(1);
  expect(lengthOfLongestSubstring("pwwkew")).toBe(3);
  expect(lengthOfLongestSubstring("")).toBe(0);
  expect(lengthOfLongestSubstring("bwf")).toBe(3);
  expect(lengthOfLongestSubstring("abba")).toBe(2);
  expect(lengthOfLongestSubstring("aab")).toBe(2);
  expect(lengthOfLongestSubstring("tmmzuxt")).toBe(5);
  expect(lengthOfLongestSubstring("biidygcc")).toBe(5);
  expect(lengthOfLongestSubstring("dvdf")).toBe(3);
});