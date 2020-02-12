/**
 * @param {number} n
 * @return {number[]}
 */
var sumZero = function (n) {
  if (n === 1) return [0]
  let ret = []

  if (1 & n) {
    for (let i = 1; i <= (n + 1) / 2; i++) {
      ret.push(i, -i)
    }
    ret[1] += ret.pop() + 2000
    ret[0] -= 2000
  } else {
    for (let i = 1; i <= n / 2; i++) {
      ret.push(i, -i)
    }
  }
  return ret;
};

module.exports = sumZero