/**
 * @param {number[]} nums
 * @return {number}
 */
var singleNumber = function (nums) {
  return nums.reduce((acc, curr) => {
    acc ^= curr;
    return acc
  }, 0)
};