/**
 * @param {number[]} A
 * @return {number}
 */
var peakIndexInMountainArray = function (A) {
  let last = -1;
  let i = 0;
  for (let n of A) {
    if (n < last) {
      return i - 1
    }
    last = n
    i++
  }
};