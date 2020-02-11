/**
 * @param {string} S
 * @return {number[]}
 */
var diStringMatch = function (S) {
  let left = 0;
  let right = 0;
  let ret = [0]
  for (let char of S) {
    if (char == 'D') {
      left -= 1
      ret.push(left)
    } else {
      right += 1
      ret.push(right)
    }
  }
  return ret.map(n => n - left)
};