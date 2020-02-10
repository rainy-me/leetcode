/**
 * @param {number[]} nums1
 * @param {number[]} nums2
 * @return {number[]}
 */
var nextGreaterElement = function (nums1, nums2) {

  const nums2Greater = new Map(
    nums2.map(
      (n, index) => {
        const slice = [...nums2].splice(index);
        const i = slice.findIndex(v => v > n);
        return [n, i === -1 ? -1 : slice[i]]
      }
    ))
  console.log(nums2Greater)
  return nums1.map(n => nums2Greater.get(n))
};