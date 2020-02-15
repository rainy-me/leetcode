/**
 * @param {string} S
 * @param {character} C
 * @return {number[]}
 */
var shortestToChar = function (S, C) {
  const ret = Array(S.length).fill(null)
  let start = -1;
  for (let i = 0; i < S.length; i++) {
    if (S[i] === C) {
      ret[i] = 0
      if (start === -1) {
        start = i
        for (let j = start; j > 0; j--) {
          ret[start - j] = j
        }
      } else {
        const middle = start + (i - start) / 2;
        for (let j = i - 1; j > start; j--) {
          ret[j] = j > middle ? i - j : j - start
        }
        start = i
      }
    }
  }
  const index = ret.indexOf(null)
  if (index !== -1) {
    for (let k = index; k < S.length; k++) {
      ret[k] = k + 1 - index
    }
  }
  return ret;
};

module.exports = shortestToChar