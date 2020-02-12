const commonChars = require('./find-common-characters.js');

test('commonChars', () => {
  expect(commonChars(["bella", "label", "roller"])).toStrictEqual(["e", "l", "l"]);
  expect(commonChars(["cool", "lock", "cook"])).toStrictEqual(["c", "o"]);
  expect(commonChars(["bbddabab", "cbcddbdd", "bbcadcab", "dabcacad", "cddcacbc", "ccbdbcba", "cbddaccc", "accdcdbb"]
  )).toStrictEqual(["b", "d"]);
});