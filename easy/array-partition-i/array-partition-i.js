/**
 * @param {number[]} nums
 * @return {number}
 */
var arrayPairSum = function (nums) {
  let array = Array(20001).fill(0);
  let sum = 0;
  let p = 0
  let len = nums.length
  for (let num of nums) {
    array[10000 + num]++;
  }
  for (let i = 0; i < 20001; i++) {
    if (p === len) break;
    while (array[i] !== 0) {
      if (p % 2 === 0) {
        sum += (i - 10000)
      }
      p++
      array[i]--
    }
  }
  return sum
};

