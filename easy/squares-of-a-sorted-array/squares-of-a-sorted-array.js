/**
 * @param {number[]} A
 * @return {number[]}
 */
var sortedSquares = function (A) {
  const ret = []
  const waiting = []
  A.forEach(n => {
    if (n < 0) {
      waiting.push(n ** 2)
    }
    else {
      const current = n ** 2
      while (waiting.length && waiting[waiting.length - 1] < current) {
        ret.push(waiting.pop())
      }
      ret.push(current)
    }
  })
  return [...ret, ...waiting.reverse()];
};

module.exports = sortedSquares