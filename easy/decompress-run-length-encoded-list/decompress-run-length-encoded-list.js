/**
 * @param {number[]} nums
 * @return {number[]}
 */
var decompressRLElist = function (nums) {
  const ret = [];
  while (nums.length) {
    const [num, count] = [nums.pop(), nums.pop()]
    ret.unshift(...Array(count).fill(num))
  }
  return ret;
};

module.exports = decompressRLElist