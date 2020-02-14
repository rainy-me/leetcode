/**
 * @param {string[]} words
 * @param {string} chars
 * @return {number}
 */
var countCharacters = function (words, chars) {
  const arr = [...chars].reduce((m, c) => {
    m.set(c, (m.get(c) || 0) + 1)
    return m;
  }, new Map());
  return words.reduce((acc, word) => {
    const m = new Map(arr);
    for (let char of word) {
      const left = m.get(char);
      if (!left) {
        return acc
      }
      m.set(char, left - 1)
    }
    return acc + word.length
  }, 0)
};

module.exports = countCharacters