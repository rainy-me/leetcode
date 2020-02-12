/**
 * @param {number[]} arr
 * @return {boolean}
 */
var uniqueOccurrences = function (arr) {
  const m = new Map()
  arr.forEach(n => {
    m.set(n, (m.get(n) || 0) + 1)
  })
  const m2 = new Map()
  for (const v of m.values()) {
    if (m2.get(v)) return false
    m2.set(v, 1)
  }
  return true
};

module.exports = uniqueOccurrences