/**
 * @param {string} s
 * @param {string} t
 * @return {boolean}
 */
var isAnagram = function (s, t) {
  if (s.length !== t.length) return false
  for (let char of s) {
    const index = t.indexOf(char)
    if (index === -1) return false
    t = t.replace(char, '')
  }
  return true
};

module.exports = isAnagram