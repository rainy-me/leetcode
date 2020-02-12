/**
 * @param {string[]} A
 * @return {string[]}
 */
var commonChars = function (A) {
  const [head, ...rest] = A
  return [...head].filter(char => {
    return rest.every((word, index) => {
      const exist = word.indexOf(char)
      if (exist !== -1) {
        rest[index] = word.slice(0, exist) + word.slice(exist + 1);
        return true
      }
      return false
    })
  })
};

module.exports = commonChars