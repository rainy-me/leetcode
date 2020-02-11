/**
 * @param {number[]} A
 * @return {number[]}
 */
var sortArrayByParity = function (A) {
  return A.reduce((acc, curr) => {
    if (curr & 1) {
      acc.push(curr)
    } else {
      acc.unshift(curr)
    }
    return acc
  }, [])
};