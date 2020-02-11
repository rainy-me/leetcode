/**
 * @param {string[]} A
 * @return {number}
 */
var minDeletionSize = function (A) {
  const len = A.length
  const size = A[0].length
  let count = 0;
  for (let i = 0; i < size; i++) {
    for (let y = 0; y < len - 1; y++) {
      if (A[y][i] > A[y + 1][i]) {
        count++
        break;
      }
    }
  }
  return count
};

//console.log({
//  curr: A[y][i],
//  next: A[y + 1][i]
//})
console.log(["rrjk", "furt", "guzm"])
console.log(minDeletionSize(["rrjk", "furt", "guzm"]))