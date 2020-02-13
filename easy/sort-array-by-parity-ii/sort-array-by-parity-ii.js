/**
 * @param {number[]} A
 * @return {number[]}
 */
var sortArrayByParityII = function (A) {
  let arr = Array(A.length);
  let evenIndex = 0;
  let oddIndex = 1;
  A.forEach(n => {
    if (n & 1) {
      arr[oddIndex] = n
      oddIndex += 2
    } else {
      arr[evenIndex] = n
      evenIndex += 2
    }
  })
  return arr;
};

module.exports = sortArrayByParityII;