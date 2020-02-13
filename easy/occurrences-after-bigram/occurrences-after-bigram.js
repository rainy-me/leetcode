/**
 * @param {string} text
 * @param {string} first
 * @param {string} second
 * @return {string[]}
 */
var findOcurrences = function (text, first, second) {
  let count = 0;
  let ret = [];
  for (word of text.split(' ')) {
    if (word === first && (count < 2)) {
      count = 1
    } else if (word === second && count === 1) {
      count++
    } else if (count === 2) {
      ret.push(word);
      count = word === first ? 1 : 0
    } else {
      count = 0
    }
  }
  return ret;
};
module.exports = findOcurrences