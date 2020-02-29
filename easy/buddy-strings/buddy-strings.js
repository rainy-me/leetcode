const twoSum = function (nums, target) {
  for (let i = 0; i < nums.length; i++) {
    for (let j = 0; j < nums.length; j++) {

      let result = nums[i] + nums[j];
      console.log(`${result}`)

      //if (result === target) {
      //  return [i, j];
      //}
    }
  }
};