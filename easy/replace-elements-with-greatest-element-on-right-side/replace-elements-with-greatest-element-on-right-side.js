/**
 * @param {number[]} arr
 * @return {number[]}
 */
var replaceElements = function (arr) {
  arr.shift()
  return [...arr.sort((a, b) => b - a), -1]
};