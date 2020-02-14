const numSpecialEquivGroups = require('./groups-of-special-equivalent-strings.js');

test('numSpecialEquivGroups', () => {
  expect(numSpecialEquivGroups(["abcd", "cdab", "cbad", "xyzz", "zzxy", "zzyx"])).toBe(3);
  expect(numSpecialEquivGroups(["abc", "acb", "bac", "bca", "cab", "cba"]
  )).toBe(3);
});