/**
 * @param {number[][]} mat
 * @param {number} k
 * @return {number[]}
 */
var kWeakestRows = function (mat, k) {
  const res = [];
  const m = mat.length;
  const n = mat[0].length;
  let last = [];
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
      //console.log({ i, j, last })
      if (mat[j][i] === 0 && (last.indexOf(j) === -1)) {
        res.push(j)
        if (res.length === k) {
          return res;
        }
        last.push(j)
      }
    }
  }
  let rest = 0; while (res.length !== k) {

    if (last.indexOf(rest) === -1) {
      res.push(rest)
    }
    rest++
  }
  return res
};
module.exports = kWeakestRows