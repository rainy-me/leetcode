/**
 * @param {number} n
 * @return {boolean}
 */
var hasAlternatingBits = function (n) {
  const s = n.toString(2)
  return !(s.includes('11') || s.includes('00'))
};

module.exports = hasAlternatingBits