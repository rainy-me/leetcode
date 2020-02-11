const uniqueMorseRepresentations = require('./unique-morse-code-words')

test('uniqueMorseRepresentations', () => {
  expect(uniqueMorseRepresentations(["gin", "zen", "gig", "msg"])).toBe(2);
})