/**
 * @param {string} s
 * @return {number}
 */
var titleToNumber = function (s) {
  return [...s].map(c => c.charCodeAt(0) - 64).reduce((acc, val) => acc * 26 + val, 0)
};


var titleToNumber2 = function (s) {
  let sum = 0;
  let head = s.length - 1
  while (head !== -1) {
    sum += (s.charCodeAt(head) - 64) * 26 ** (s.length - 1 - head)
    head--
  }
  return sum
};

module.exports = titleToNumber