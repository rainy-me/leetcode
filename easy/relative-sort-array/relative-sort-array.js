/**
 * @param {number[]} arr1
 * @param {number[]} arr2
 * @return {number[]}
 */
var relativeSortArray = function (arr1, arr2) {
  const arr = []
  arr2.forEach(a2 => {
    let index = arr1.indexOf(a2)
    while (index !== -1) {
      arr.push(a2)
      arr1.splice(index, 1)
      index = arr1.indexOf(a2)
    }
  })

  return [...arr, ...arr1.sort((a, b) => a - b)]
};

module.exports = relativeSortArray;