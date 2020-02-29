const twoSum = function (nums, target) {
  let dic = {}
  for (let [i, num] of nums.entries()) {
    if ((num) in dic) {
      return [dic[num], i]

    } else {
      dic[target - num] = i
    }
  }
};