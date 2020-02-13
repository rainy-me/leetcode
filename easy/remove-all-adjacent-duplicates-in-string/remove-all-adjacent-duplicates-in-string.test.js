const removeDuplicates = require('./remove-all-adjacent-duplicates-in-string.js');

test('removeDuplicates', () => {
  expect(removeDuplicates("abbaca")).toBe("ca");
});