/**
 * @param {string} s
 * @param {string} t
 * @return {character}
 */
var findTheDifference = function (s, t) {
  const table = Array(26).fill(0)
  for (const c1 of s) {
    table[c1.charCodeAt(0) - 97]++
  }
  for (const c2 of t) {
    table[c2.charCodeAt(0) - 97]--
  }
  return String.fromCharCode(97 + table.findIndex(n => n !== 0))
};


var findTheDifference2 = function (s, t) {
  return String.fromCharCode([...s, ...t].reduce((a, c) => a ^ c.charCodeAt(0), 0))
};
module.exports = findTheDifference;