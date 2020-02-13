const findOcurrences = require('./occurrences-after-bigram.js');

test('findOcurrences', () => {
  expect(findOcurrences("alice is a good girl she is a good student", "a", "good")).toStrictEqual(["girl", "student"]);
  expect(findOcurrences("we will we will rock you", "we", "will")).toStrictEqual(["we", "rock"]);
  expect(findOcurrences("obo jvezipre obo jnvavldde jvezipre jvezipre jnvavldde jvezipre jvezipre jvezipre y jnvavldde jnvavldde obo jnvavldde jnvavldde obo jnvavldde jnvavldde jvezipre",
    "jnvavldde",
    "y"
  )).toStrictEqual([]);

  expect(findOcurrences(
    "jkypmsxd jkypmsxd kcyxdfnoa jkypmsxd kcyxdfnoa jkypmsxd kcyxdfnoa kcyxdfnoa jkypmsxd kcyxdfnoa",
    "kcyxdfnoa",
    "jkypmsxd"

  )).toStrictEqual(["kcyxdfnoa", "kcyxdfnoa", "kcyxdfnoa"]);

});