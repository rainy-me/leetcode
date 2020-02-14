/**
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var moveZeroes = function (nums) {
  for (let i = nums.length; i--;) {
    if (nums[i] === 0) {
      nums.splice(i, 1);
      nums.push(0);
    }
  }
};

var moveZeroesDoesNotWorkToo = function (nums) {
  [...Array(nums.length).keys()].map((i) => {
    if (nums[i] === 0) {
      nums.splice(i, 1);
      nums.push(0);
    }
  })
};

var moveZeroesDoesNotWork = function (nums) {
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] === 0) {
      nums.splice(i, 1);
      nums.push(0);
    }
  }
};


module.exports = moveZeroes