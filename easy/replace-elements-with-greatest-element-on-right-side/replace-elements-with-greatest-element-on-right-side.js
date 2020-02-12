/**
 * @param {number[]} arr
 * @return {number[]}
 */
var replaceElements = function (arr) {
  let prev = -1;
  for (let i = arr.length - 1; i >= 0; i--) {
    let cur = arr[i];
    arr[i] = prev;
    prev = Math.max(prev, cur);
  }
  return arr;
};

module.exports = replaceElements;
