/**
 * @param {number[]} A
 * @return {number}
 */
var repeatedNTimes = function (A) {
  const m = new Map()
  for (el of A) {
    const count = m.get(el)
    if (count == undefined) {
      m.set(el, 1)
    } else if (count + 1 === 2) {
      return el
    }
  }
};

module.exports = repeatedNTimes;